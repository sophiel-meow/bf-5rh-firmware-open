#![no_std]
#![no_main]

use bf5rh_abi::{Api, AppEvent, AppResult};

const KIND_SINGLE: u8 = 2;
const KIND_REPEAT: u8 = 4;
const KEY_ASTERISK: u8 = 10;
const KEY_POUND: u8 = 11;
const KEY_MENU: u8 = 12;
const KEY_EXIT: u8 = 13;
const KEY_UP: u8 = 14;
const KEY_DOWN: u8 = 15;
const KEY_SIDE1: u8 = 19;
const KEY_SIDE2: u8 = 20;

const F5: u8 = 0; // 5x8

const BLACK: u16 = 0x0000;
const WHITE: u16 = 0xFFFF;
const GRAY: u16 = 0x8410;
const GRID: u16 = 0x2124;
const TRACE: u16 = 0x07E0; // bright green
const PEAKLINE: u16 = 0x8400; // dim yellow
const TRIGLINE: u16 = 0xF800; // red
const MARKER: u16 = 0xFFE0; // yellow
const CYAN: u16 = 0x07FF;

const SCREEN_W: usize = 160;
const HEADER_Y: i16 = 1;
const GRAPH_TOP: i16 = 11;
const GRAPH_BOTTOM: i16 = 71;
const WF_TOP: i16 = 73;
const WF_H: usize = 32;
const LINE1_Y: i16 = 110;
const LINE2_Y: i16 = 119;

const WF_POOL: usize = 1024;

const WF_FLOOR_HEADROOM_DB: u16 = 6;

const WF_PALETTE: [u16; 16] = [
    0x0008, 0x0010, 0x0016, 0x001B, 0x001F, 0x021F, 0x04DF, 0x07FF, 0x07F3,
    0x07E0, 0x4FE0, 0x9FE0, 0xFFE0, 0xFD00, 0xF800, 0xFFFF,
];

const MAX_BINS: usize = 128;

const SCAN_STEPS_HZ: [u32; 15] = [
    10, 100, 500, 1_000, 2_500, 5_000, 6_250, 8_330, 10_000, 12_500, 15_000,
    20_000, 25_000, 50_000, 100_000,
];
const DEFAULT_SCAN_STEP_INDEX: u8 = 8; // 10 kHz

const STEPS_COUNT_TABLE: [u16; 4] = [128, 64, 32, 16];
const DEFAULT_STEPS_INDEX: u8 = 1; // 64 bins

const PAN_STEP_MIN_HZ: u32 = 100_000;
const PAN_STEP_MAX_HZ: u32 = 2_000_000;
const DEFAULT_PAN_STEP_HZ: u32 = 800_000;

const RSSI_CEILING_MIN: i16 = -120;
const RSSI_CEILING_MAX: i16 = -20;
const RSSI_CEILING_STEP: i16 = 4;
const DEFAULT_RSSI_CEILING: i16 = -60;

const DEFAULT_TRIGGER_MARGIN_DB: i16 = 16;
const FLOOR_PERCENT: u32 = 50;
const NOISE_TOP_PERCENT: u32 = 95;
const TRIGGER_MARGIN_MIN_DB: i16 = 2;
const TRIGGER_MARGIN_MAX_DB: i16 = 40;
const TRIGGER_MARGIN_STEP_DB: i16 = 2;
const INITIAL_FLOOR_DBM: i16 = -100;

const BLACKLIST_SENTINEL: u16 = u16::MAX;
const PEAK_HOLD_STEPS: u16 = 1024;
const LISTEN_HANG_100US: u32 = 3000;
const LISTEN_DROP_DB: u16 = 10;
const DEFAULT_AGC_RANK: i8 = 0;
const AGC_RANK_MAX: i8 = 7;
const LISTEN_REFRESH_100US: u32 = 90_000;

const RSSI_DBM_BASE: i32 = 170;
const DBM_FLOOR: i32 = -141;

const DEFAULT_DWELL_100US: u16 = 30;
const DWELL_MIN_100US: u16 = 2;
const DWELL_MAX_100US: u16 = 120;

const TICK_BUDGET_100US: u32 = 500;

const FREQ_INPUT_DIGITS: usize = 6;

struct State {
    window_start_hz: u32,
    scan_step_index: u8,
    steps_index: u8,
    pan_step_hz: u32,
    dwell_100us: u16,

    scan_pos: u16,
    rssi_bins: [u16; MAX_BINS],

    rssi_ceiling: i16,
    trigger_level: u16,
    release_level: u16,
    trigger_margin_db: i16,

    hist: [u8; 256],
    sweep_samples: u16,
    sweep_max: u16,
    sweep_max_bin: u16,
    floor_raw: u16,
    noise_top_raw: u16,
    noise_max_raw: u16,
    parked_floor_raw: u16,
    agc_rank: i8,

    peak_bin: u16,
    peak_valid: bool,
    peak_rssi: u16,
    peak_age: u16,

    auto_listen: bool,
    listening: bool,
    listen_bin: u16,
    listen_max_raw: u16,
    listen_above_100us: u32,
    listen_rssi: u16,
    listen_start_100us: u32,
    listen_inhibit: bool,

    entering_freq: bool,
    input: [u8; FREQ_INPUT_DIGITS],
    input_len: usize,

    trace_y: [u8; SCREEN_W],
    peak_y: [u8; SCREEN_W],

    wf_head: u8,
    wf_fill: u8,

    dirty: bool,
    status_dirty: bool,
    full_redraw: bool,
    sweep_start_100us: u32,
}

static mut STATE: State = State {
    window_start_hz: 0,
    scan_step_index: DEFAULT_SCAN_STEP_INDEX,
    steps_index: DEFAULT_STEPS_INDEX,
    pan_step_hz: DEFAULT_PAN_STEP_HZ,
    dwell_100us: DEFAULT_DWELL_100US,
    scan_pos: 0,
    rssi_bins: [0; MAX_BINS],
    rssi_ceiling: DEFAULT_RSSI_CEILING,
    trigger_level: 0,
    release_level: 0,
    trigger_margin_db: DEFAULT_TRIGGER_MARGIN_DB,
    hist: [0; 256],
    sweep_samples: 0,
    sweep_max: 0,
    sweep_max_bin: 0,
    floor_raw: 0,
    noise_top_raw: 0,
    noise_max_raw: 0,
    parked_floor_raw: 0,
    agc_rank: DEFAULT_AGC_RANK,
    peak_bin: 0,
    peak_valid: false,
    peak_rssi: 0,
    peak_age: 0,
    auto_listen: false,
    listening: false,
    listen_bin: 0,
    listen_max_raw: 0,
    listen_above_100us: 0,
    listen_rssi: 0,
    listen_start_100us: 0,
    listen_inhibit: false,
    entering_freq: false,
    input: [0; FREQ_INPUT_DIGITS],
    input_len: 0,
    trace_y: [GRAPH_BOTTOM as u8; SCREEN_W],
    peak_y: [GRAPH_BOTTOM as u8; SCREEN_W],
    wf_head: 0,
    wf_fill: 0,
    dirty: true,
    status_dirty: false,
    full_redraw: true,
    sweep_start_100us: 0,
};

static mut ROW: [u16; SCREEN_W] = [0; SCREEN_W];

static mut WF: [u8; WF_POOL] = [0; WF_POOL];

fn wf() -> &'static mut [u8; WF_POOL] {
    unsafe { &mut *core::ptr::addr_of_mut!(WF) }
}

static mut API_PTR: *const Api = core::ptr::null();

fn state() -> &'static mut State {
    unsafe { &mut *core::ptr::addr_of_mut!(STATE) }
}

fn rssi_to_dbm(raw: u16) -> i32 {
    raw as i32 - RSSI_DBM_BASE
}

fn dbm_to_rssi(dbm: i16) -> u16 {
    (dbm as i32 + RSSI_DBM_BASE).clamp(0, u16::MAX as i32) as u16
}

fn max_trigger_for(ceiling_dbm: i16) -> u16 {
    dbm_to_rssi(ceiling_dbm)
}

fn rssi_to_y(rssi: u16, ceiling_dbm: i16) -> i16 {
    let ceiling = (ceiling_dbm as i32).max(DBM_FLOOR + 1);
    let dbm = rssi_to_dbm(rssi).clamp(DBM_FLOOR, ceiling);
    let height = (GRAPH_BOTTOM - GRAPH_TOP) as i32;
    let span = ceiling - DBM_FLOOR;
    (GRAPH_BOTTOM as i32 - (dbm - DBM_FLOOR) * height / span) as i16
}

impl State {
    fn scan_step_hz(&self) -> u32 {
        SCAN_STEPS_HZ[self.scan_step_index as usize]
    }

    fn bins(&self) -> u16 {
        STEPS_COUNT_TABLE[self.steps_index as usize]
    }

    fn span_hz(&self) -> u32 {
        self.bins() as u32 * self.scan_step_hz()
    }

    fn freq_at(&self, pos: u16) -> u32 {
        self.window_start_hz + pos as u32 * self.scan_step_hz()
    }

    fn centre_window(&mut self, centre_hz: u32) {
        let half = (self.bins() as u32 / 2) * self.scan_step_hz();
        self.window_start_hz = centre_hz.saturating_sub(half);
    }

    fn relaunch(&mut self) {
        self.rssi_bins = [0; MAX_BINS];
        self.scan_pos = 0;
        self.peak_valid = false;
        self.peak_rssi = 0;
        self.peak_age = 0;
        self.peak_y = [GRAPH_BOTTOM as u8; SCREEN_W];
        self.noise_max_raw = 0;
        self.parked_floor_raw = 0;
        self.wf_head = 0;
        self.wf_fill = 0;
        self.reset_histogram();
        self.dirty = true;
    }

    fn wf_stride(&self) -> usize {
        (self.bins() as usize) / 2
    }

    fn wf_rows(&self) -> usize {
        (WF_POOL / self.wf_stride()).min(WF_H).max(1)
    }

    fn wf_row_h(&self) -> i16 {
        (WF_H / self.wf_rows()) as i16
    }

    fn wf_push(&mut self) {
        let bins = self.bins() as usize;
        let rows = self.wf_rows();
        let stride = self.wf_stride();
        let head = (self.wf_head as usize + 1) % rows;
        self.wf_head = head as u8;
        if (self.wf_fill as usize) < rows {
            self.wf_fill += 1;
        }
        let base = head * stride;
        let wf = wf();
        let lo = self.floor_raw.saturating_sub(WF_FLOOR_HEADROOM_DB);
        let hi = dbm_to_rssi(self.rssi_ceiling).max(lo + 1);
        let span = (hi - lo) as u32;
        for i in 0..bins {
            let v = self.bin_value(i);
            let idx = if v <= lo {
                0u8
            } else {
                (((v - lo) as u32 * 15 / span).min(15)) as u8
            };
            let b = &mut wf[base + i / 2];
            *b = if i & 1 == 0 {
                (*b & 0xF0) | idx
            } else {
                (*b & 0x0F) | (idx << 4)
            };
        }
    }

    fn reset_histogram(&mut self) {
        self.hist = [0; 256];
        self.sweep_samples = 0;
        self.sweep_max = 0;
        self.sweep_max_bin = 0;
    }

    fn percentile(&self, pct: u32, fallback: u16) -> u16 {
        if self.sweep_samples == 0 {
            return fallback;
        }
        let target = (self.sweep_samples as u32 * pct / 100).max(1);
        let mut acc: u32 = 0;
        for (v, &count) in self.hist.iter().enumerate() {
            acc += count as u32;
            if acc >= target {
                return v as u16;
            }
        }
        fallback
    }

    fn latch_ref(&self) -> u16 {
        self.noise_top_raw.max(self.noise_max_raw)
    }

    fn apply_trigger(&mut self) {
        let cap = max_trigger_for(self.rssi_ceiling) as i32;
        let margin = self.trigger_margin_db as i32;
        let latch = self.latch_ref() as i32 + margin;
        self.trigger_level = latch.clamp(0, cap) as u16;
        let floor = self.floor_raw.max(self.parked_floor_raw);
        let release = floor as i32 + margin / 2;
        self.release_level = release.clamp(0, cap) as u16;
    }

    fn bin_value(&self, i: usize) -> u16 {
        let v = self.rssi_bins[i];
        if v == BLACKLIST_SENTINEL {
            0
        } else {
            v
        }
    }

    fn column_value(&self, x: usize) -> u16 {
        let bins = self.bins() as usize;
        if bins <= 1 {
            return self.bin_value(0);
        }
        let num = x * (bins - 1) * 256 / (SCREEN_W - 1);
        let i = num >> 8;
        let f = (num & 0xFF) as u32;
        let a = self.bin_value(i.min(bins - 1)) as u32;
        let b = self.bin_value((i + 1).min(bins - 1)) as u32;
        ((a * (256 - f) + b * f) >> 8) as u16
    }
}

#[no_mangle]
pub extern "C" fn app_entry(api: &Api, ev: AppEvent) -> AppResult {
    unsafe {
        API_PTR = api as *const Api;
    }
    match ev {
        AppEvent::Enter => enter(api),
        AppEvent::Key { id, kind } => key(api, id, kind),
        AppEvent::Tick { dt_100us } => tick(api, dt_100us),
        AppEvent::Draw => draw(api),
        AppEvent::Leave => leave(api),
    }
}

fn enter(api: &Api) -> AppResult {
    let st = state();
    let centre = (api.get_master_freq)();

    st.scan_step_index = DEFAULT_SCAN_STEP_INDEX;
    st.steps_index = DEFAULT_STEPS_INDEX;
    st.pan_step_hz = DEFAULT_PAN_STEP_HZ;
    st.dwell_100us = DEFAULT_DWELL_100US;
    st.rssi_ceiling = DEFAULT_RSSI_CEILING;
    st.trigger_margin_db = DEFAULT_TRIGGER_MARGIN_DB;
    st.floor_raw = dbm_to_rssi(INITIAL_FLOOR_DBM);
    st.noise_top_raw = st.floor_raw;
    st.noise_max_raw = st.floor_raw;
    st.apply_trigger();
    st.auto_listen = false;
    st.listening = false;
    st.listen_inhibit = false;
    st.entering_freq = false;
    st.input_len = 0;
    st.trace_y = [GRAPH_BOTTOM as u8; SCREEN_W];
    st.centre_window(centre);
    st.relaunch();
    st.full_redraw = true;
    st.sweep_start_100us = (api.uptime_100us)();

    (api.set_modulation)(0); // FM
    (api.set_bandwidth)(true);
    (api.enter_rx)();
    (api.set_holds_rx)(false);
    (api.set_af_out)(false);
    (api.set_agc_fix)(st.agc_rank);
    (api.set_backlight_hold)(true);
    AppResult::Continue
}

fn leave(api: &Api) -> AppResult {
    (api.set_af_out)(false);
    (api.set_agc_fix)(-1);
    (api.set_backlight_hold)(false);
    AppResult::Continue
}

fn dwell(api: &Api, units: u16) {
    let t0 = (api.uptime_100us)();
    while (api.uptime_100us)().wrapping_sub(t0) < units as u32 {}
}

fn tick(api: &Api, _dt_100us: u32) -> AppResult {
    let st = state();
    if st.entering_freq {
        return AppResult::Continue;
    }
    if st.listening {
        poll_listen(api, st);
        return AppResult::Continue;
    }

    let t0 = (api.uptime_100us)();
    let mut swept = false;
    loop {
        step_scan(api, st);
        if st.scan_pos == 0 {
            swept = true;
            break;
        }
        if (api.uptime_100us)().wrapping_sub(t0) >= TICK_BUDGET_100US {
            break;
        }
    }

    let now = (api.uptime_100us)();
    if swept {
        st.sweep_start_100us = now;
        st.floor_raw = st.percentile(FLOOR_PERCENT, st.floor_raw);
        st.noise_top_raw = st.percentile(NOISE_TOP_PERCENT, st.noise_top_raw);
        st.apply_trigger();
        st.wf_push();
        st.dirty = true;

        let over = st.sweep_max >= st.trigger_level;
        let latch = st.auto_listen && !st.listen_inhibit && over;
        st.listen_inhibit = false;
        if over {
        } else {
            st.noise_max_raw =
                st.sweep_max.max(st.noise_max_raw.saturating_sub(1));
        }
        if latch {
            let freq = st.freq_at(st.sweep_max_bin);
            let bin = st.sweep_max_bin;
            start_listening(api, st, bin, freq);
        }
        st.reset_histogram();
    }
    AppResult::Continue
}

fn step_scan(api: &Api, st: &mut State) {
    let pos = st.scan_pos;
    let freq = st.freq_at(pos);

    if st.rssi_bins[pos as usize] != BLACKLIST_SENTINEL {
        (api.retune_rx)(freq, true);
        dwell(api, st.dwell_100us);
        let rssi = (api.read_rssi)();
        st.rssi_bins[pos as usize] = rssi;
        let bucket = rssi.min(255) as usize;
        st.hist[bucket] = st.hist[bucket].saturating_add(1);
        st.sweep_samples = st.sweep_samples.saturating_add(1);
        if st.sweep_samples == 1 || rssi > st.sweep_max {
            st.sweep_max = rssi;
            st.sweep_max_bin = pos;
        }

        if !st.peak_valid
            || rssi > st.peak_rssi
            || st.peak_age >= PEAK_HOLD_STEPS
        {
            st.peak_valid = true;
            st.peak_bin = pos;
            st.peak_rssi = rssi;
            st.peak_age = 0;
        } else {
            st.peak_age = st.peak_age.saturating_add(1);
        }
    }

    st.scan_pos += 1;
    if st.scan_pos >= st.bins() {
        st.scan_pos = 0;
    }
}

fn start_listening(api: &Api, st: &mut State, bin: u16, freq: u32) {
    let now = (api.uptime_100us)();
    st.listening = true;
    st.listen_bin = bin;
    st.listen_max_raw = 0;
    st.listen_above_100us = now;
    st.listen_start_100us = now;
    st.status_dirty = true;
    (api.retune_rx)(freq, true);
    (api.set_af_out)(true);
}

fn stop_listening(api: &Api, st: &mut State) {
    st.listening = false;
    let held = (api.uptime_100us)().wrapping_sub(st.listen_start_100us);
    st.sweep_start_100us = st.sweep_start_100us.wrapping_add(held);
    st.status_dirty = true;
    (api.set_af_out)(false);
}

fn poll_listen(api: &Api, st: &mut State) {
    let now = (api.uptime_100us)();
    let pos = st.listen_bin as usize;
    let rssi = (api.read_rssi)();
    st.listen_rssi = rssi;
    st.status_dirty = true;
    if st.rssi_bins[pos] != BLACKLIST_SENTINEL {
        st.rssi_bins[pos] = rssi;
    }

    if now.wrapping_sub(st.listen_start_100us) >= LISTEN_REFRESH_100US {
        st.listen_inhibit = true;
        stop_listening(api, st);
        return;
    }

    st.listen_max_raw = st.listen_max_raw.max(rssi);
    let above_floor = rssi >= st.release_level;
    let near_peak = rssi + LISTEN_DROP_DB >= st.listen_max_raw;
    if above_floor && near_peak {
        st.listen_above_100us = now;
    } else if now.wrapping_sub(st.listen_above_100us) >= LISTEN_HANG_100US {
        st.parked_floor_raw = rssi;
        st.apply_trigger();
        stop_listening(api, st);
    }
}

fn key(api: &Api, id: u8, kind: u8) -> AppResult {
    let st = state();

    if st.entering_freq {
        return freq_input_key(api, st, id, kind);
    }
    if kind == KIND_SINGLE && id == KEY_EXIT {
        return AppResult::Exit;
    }
    if st.listening && kind == KIND_SINGLE {
        st.listen_inhibit = true;
        stop_listening(api, st);
    }
    if kind != KIND_SINGLE && kind != KIND_REPEAT {
        return AppResult::Continue;
    }
    let repeatable = matches!(id, KEY_UP | KEY_DOWN);
    if kind == KIND_REPEAT && !repeatable {
        return AppResult::Continue;
    }

    match id {
        KEY_UP => pan(st, true),
        KEY_DOWN => pan(st, false),
        1 => change_scan_step(st, true),
        7 => change_scan_step(st, false),
        2 => change_pan_step(st, true),
        8 => change_pan_step(st, false),
        3 => change_ceiling(st, true),
        9 => change_ceiling(st, false),
        KEY_ASTERISK => change_trigger(st, true),
        KEY_POUND => change_trigger(st, false),
        4 => {
            st.steps_index =
                (st.steps_index + 1) % STEPS_COUNT_TABLE.len() as u8;
            st.relaunch();
        }
        5 => {
            st.entering_freq = true;
            st.input_len = 0;
            st.dirty = true;
        }
        0 => change_dwell(st, true),
        6 => change_dwell(st, false),
        KEY_MENU => {
            st.auto_listen = !st.auto_listen;
            if st.auto_listen {
                st.listen_inhibit = false;
            }
            if !st.auto_listen && st.listening {
                stop_listening(api, st);
            }
            st.dirty = true;
        }
        KEY_SIDE1 => blacklist_peak(st),
        KEY_SIDE2 => {
            st.agc_rank = if st.agc_rank < 0 {
                0
            } else if st.agc_rank >= AGC_RANK_MAX {
                -1
            } else {
                st.agc_rank + 1
            };
            (api.set_agc_fix)(st.agc_rank);
            st.relaunch();
        }
        _ => return AppResult::Continue,
    }
    AppResult::Continue
}

fn pan(st: &mut State, up: bool) {
    let step = st.pan_step_hz;
    st.window_start_hz = if up {
        st.window_start_hz.saturating_add(step)
    } else {
        st.window_start_hz.saturating_sub(step)
    };
    st.relaunch();
}

fn change_scan_step(st: &mut State, up: bool) {
    let len = SCAN_STEPS_HZ.len() as u8;
    st.scan_step_index = if up {
        (st.scan_step_index + 1) % len
    } else {
        (st.scan_step_index + len - 1) % len
    };
    st.relaunch();
}

fn change_pan_step(st: &mut State, up: bool) {
    st.pan_step_hz = if up {
        (st.pan_step_hz * 2).min(PAN_STEP_MAX_HZ)
    } else {
        (st.pan_step_hz / 2).max(PAN_STEP_MIN_HZ)
    };
    st.dirty = true;
}

fn change_ceiling(st: &mut State, up: bool) {
    st.rssi_ceiling = if up {
        (st.rssi_ceiling + RSSI_CEILING_STEP).min(RSSI_CEILING_MAX)
    } else {
        (st.rssi_ceiling - RSSI_CEILING_STEP).max(RSSI_CEILING_MIN)
    };
    st.apply_trigger();
    st.peak_y = [GRAPH_BOTTOM as u8; SCREEN_W];
    st.dirty = true;
}

fn change_trigger(st: &mut State, up: bool) {
    st.trigger_margin_db = if up {
        (st.trigger_margin_db + TRIGGER_MARGIN_STEP_DB)
            .min(TRIGGER_MARGIN_MAX_DB)
    } else {
        (st.trigger_margin_db - TRIGGER_MARGIN_STEP_DB)
            .max(TRIGGER_MARGIN_MIN_DB)
    };
    st.apply_trigger();
    st.dirty = true;
}

fn change_dwell(st: &mut State, up: bool) {
    st.dwell_100us = if up {
        (st.dwell_100us + 2).min(DWELL_MAX_100US)
    } else {
        st.dwell_100us.saturating_sub(2).max(DWELL_MIN_100US)
    };
    st.dirty = true;
}

fn blacklist_peak(st: &mut State) {
    if st.peak_valid {
        st.rssi_bins[st.peak_bin as usize] = BLACKLIST_SENTINEL;
        st.peak_valid = false;
        st.peak_rssi = 0;
        st.dirty = true;
    }
}

fn freq_input_key(api: &Api, st: &mut State, id: u8, kind: u8) -> AppResult {
    if kind != KIND_SINGLE {
        return AppResult::Continue;
    }
    if id <= 9 {
        if st.input_len < FREQ_INPUT_DIGITS {
            st.input[st.input_len] = id;
            st.input_len += 1;
        }
        st.dirty = true;
        if st.input_len == FREQ_INPUT_DIGITS {
            commit_freq(api, st);
        }
        return AppResult::Continue;
    }
    match id {
        KEY_MENU => commit_freq(api, st),
        KEY_EXIT => {
            if st.input_len > 0 {
                st.input_len = 0;
            } else {
                st.entering_freq = false;
            }
            st.dirty = true;
        }
        _ => {}
    }
    AppResult::Continue
}

fn commit_freq(api: &Api, st: &mut State) {
    if st.input_len > 0 {
        let mut khz: u32 = 0;
        for i in 0..FREQ_INPUT_DIGITS {
            let d = if i < st.input_len { st.input[i] } else { 0 };
            khz = khz * 10 + d as u32;
        }
        st.centre_window(khz * 1000);
    }
    st.input_len = 0;
    st.entering_freq = false;
    st.relaunch();
    st.full_redraw = true;
    st.sweep_start_100us = (api.uptime_100us)();
}

struct Buf {
    b: [u8; 40],
    n: usize,
}

impl Buf {
    fn new() -> Buf {
        Buf { b: [0; 40], n: 0 }
    }

    fn ch(&mut self, c: u8) {
        if self.n < self.b.len() {
            self.b[self.n] = c;
            self.n += 1;
        }
    }

    fn s(&mut self, text: &[u8]) {
        for &c in text {
            self.ch(c);
        }
    }

    fn u(&mut self, api: &Api, v: u32) {
        let mut tmp = [0u8; 12];
        let n = (api.fmt_u32)(v, tmp.as_mut_ptr()) as usize;
        self.s(&tmp[..n.min(tmp.len())]);
    }

    fn i(&mut self, api: &Api, v: i32) {
        if v < 0 {
            self.ch(b'-');
            self.u(api, (-v) as u32);
        } else {
            self.u(api, v as u32);
        }
    }

    fn freq(&mut self, api: &Api, hz: u32) {
        let mut tmp = [0u8; 16];
        let n = (api.fmt_freq)(hz, tmp.as_mut_ptr()) as usize;
        self.s(&tmp[..n.min(tmp.len())]);
    }

    fn as_slice(&self) -> &[u8] {
        &self.b[..self.n]
    }
}

fn text(api: &Api, x: i16, y: i16, buf: &Buf, fg: u16) {
    let s = buf.as_slice();
    (api.draw_text)(x, y, s.as_ptr(), s.len() as u16, fg, BLACK, F5);
}

fn text_line(api: &Api, y: i16, buf: &Buf, fg: u16) {
    (api.fill_rect)(0, y, SCREEN_W as u16, 8, BLACK);
    text(api, 0, y, buf, fg);
}

fn fill_color(y: i16) -> u16 {
    let h = (GRAPH_BOTTOM - GRAPH_TOP) as i32;
    let t = (GRAPH_BOTTOM - y) as i32; // 0 at the bottom, h at the top
    let g = (6 + t * 22 / h) as u16; // 6-bit green
    let b = (5 + t * 11 / h) as u16; // 5-bit blue
    (g << 5) | b
}

fn recompute_trace(st: &mut State) {
    for x in 0..SCREEN_W {
        let y = rssi_to_y(st.column_value(x), st.rssi_ceiling);
        st.trace_y[x] = y as u8;
        if y <= st.peak_y[x] as i16 {
            st.peak_y[x] = y as u8;
        } else if st.peak_y[x] < GRAPH_BOTTOM as u8 {
            st.peak_y[x] += 1;
        }
    }
}

fn render_graph(api: &Api, st: &State) {
    let row = unsafe { &mut *core::ptr::addr_of_mut!(ROW) };
    let trig_y = rssi_to_y(st.trigger_level, st.rssi_ceiling);
    let marker_x = if st.peak_valid {
        bin_to_column(st, st.peak_bin) as i32
    } else {
        -1
    };

    for y in GRAPH_TOP..=GRAPH_BOTTOM {
        let fill = fill_color(y);
        let grid_row = (y - GRAPH_TOP) % 22 == 0 || y == GRAPH_BOTTOM;
        for x in 0..SCREEN_W {
            let ty = st.trace_y[x] as i16;
            row[x] = if y == ty {
                TRACE
            } else if y > ty {
                fill
            } else if y == st.peak_y[x] as i16 {
                PEAKLINE
            } else if y == trig_y {
                if x % 4 < 2 {
                    TRIGLINE
                } else {
                    BLACK
                }
            } else if x as i32 == marker_x {
                MARKER
            } else if grid_row || x % 32 == 0 {
                GRID
            } else {
                BLACK
            };
        }
        (api.blit)(0, y, SCREEN_W as u16, 1, row.as_ptr());
    }
}

fn render_waterfall(api: &Api, st: &State) {
    let row = unsafe { &mut *core::ptr::addr_of_mut!(ROW) };
    let wf = wf();
    let rows = st.wf_rows();
    let rh = st.wf_row_h();
    let stride = st.wf_stride();
    let bins = st.bins() as usize;
    let mut y = WF_TOP;

    for i in 0..rows {
        if i < st.wf_fill as usize {
            let r = (st.wf_head as usize + rows - i) % rows;
            let base = r * stride;
            for x in 0..SCREEN_W {
                let b = if bins <= 1 {
                    0
                } else {
                    x * (bins - 1) / (SCREEN_W - 1)
                };
                let byte = wf[base + b / 2];
                let idx = if b & 1 == 0 { byte & 0x0F } else { byte >> 4 };
                row[x] = WF_PALETTE[idx as usize];
            }
        } else {
            row.fill(BLACK);
        }
        for _ in 0..rh {
            (api.blit)(0, y, SCREEN_W as u16, 1, row.as_ptr());
            y += 1;
        }
    }
}

fn bin_to_column(st: &State, bin: u16) -> usize {
    let bins = st.bins() as usize;
    if bins <= 1 {
        return 0;
    }
    (bin as usize * (SCREEN_W - 1) / (bins - 1)).min(SCREEN_W - 1)
}

fn draw(api: &Api) -> AppResult {
    let st = state();
    if !st.dirty && !st.full_redraw {
        if st.status_dirty {
            draw_status(api, st);
            st.status_dirty = false;
        }
        return AppResult::Continue;
    }

    if st.full_redraw {
        (api.fill_rect)(0, 0, SCREEN_W as u16, 128, BLACK);
        st.full_redraw = false;
    }

    recompute_trace(st);
    render_graph(api, st);
    (api.fill_rect)(0, GRAPH_BOTTOM + 1, SCREEN_W as u16, 1, GRID);
    render_waterfall(api, st);
    draw_header(api, st);
    draw_status(api, st);

    st.dirty = false;
    st.status_dirty = false;
    AppResult::Continue
}

fn draw_header(api: &Api, st: &State) {
    (api.fill_rect)(0, HEADER_Y, SCREEN_W as u16, 8, BLACK);

    let mut left = Buf::new();
    left.freq(api, st.window_start_hz);
    text(api, 0, HEADER_Y, &left, WHITE);

    let mut right = Buf::new();
    right.freq(api, st.window_start_hz + st.span_hz());
    let w = right.as_slice().len() as i16 * 5;
    text(api, SCREEN_W as i16 - w, HEADER_Y, &right, WHITE);
}

fn draw_status(api: &Api, st: &State) {
    let mut line1 = Buf::new();
    if st.entering_freq {
        line1.s(b"F ");
        for i in 0..FREQ_INPUT_DIGITS {
            if i == 3 {
                line1.ch(b'.');
            }
            line1.ch(if i < st.input_len {
                b'0' + st.input[i]
            } else {
                b'-'
            });
        }
        line1.s(b" MHz");
        text_line(api, LINE1_Y, &line1, MARKER);
    } else if st.listening {
        line1.s(b"RX ");
        line1.freq(api, st.freq_at(st.listen_bin));
        line1.ch(b' ');
        line1.i(api, rssi_to_dbm(st.listen_rssi));
        line1.s(b"dBm");
        text_line(api, LINE1_Y, &line1, TRACE);
    } else if st.peak_valid {
        line1.s(b"P ");
        line1.freq(api, st.freq_at(st.peak_bin));
        line1.ch(b' ');
        line1.i(api, rssi_to_dbm(st.peak_rssi));
        line1.s(b"dBm");
        text_line(api, LINE1_Y, &line1, CYAN);
    } else {
        line1.s(b"PAN ");
        line1.u(api, st.pan_step_hz / 1000);
        line1.ch(b'k');
        text_line(api, LINE1_Y, &line1, GRAY);
    }

    let mut line2 = Buf::new();
    line2.s(b"ST");
    let step = st.scan_step_hz();
    if step >= 1000 {
        line2.u(api, step / 1000);
        line2.ch(b'k');
    } else {
        line2.u(api, step);
    }
    line2.s(b" B");
    line2.u(api, st.bins() as u32);
    line2.s(b" D");
    line2.u(api, st.dwell_100us as u32 / 10);
    line2.ch(b'.');
    line2.u(api, st.dwell_100us as u32 % 10);
    line2.s(b" T+");
    line2.u(api, st.trigger_margin_db as u32);
    line2.s(b" C");
    line2.i(api, st.rssi_ceiling as i32);
    line2.s(b" G");
    if st.agc_rank < 0 {
        line2.ch(b'A');
    } else {
        line2.u(api, st.agc_rank as u32);
    }
    if st.auto_listen {
        line2.s(b" A");
    }
    if st.listen_inhibit {
        line2.ch(b'!');
    }
    text_line(api, LINE2_Y, &line2, GRAY);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        let ptr = API_PTR;
        if !ptr.is_null() {
            ((*ptr).app_fault)(0);
        }
    }
    loop {}
}
