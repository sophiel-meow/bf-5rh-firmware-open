#![no_std]
#![no_main]

use bf5rh_abi::{Api, AppEvent, AppResult};

const KIND_SINGLE: u8 = 2;
const KIND_LONG: u8 = 3;
const KEY_MENU: u8 = 12;
const KEY_EXIT: u8 = 13;
const KEY_UP: u8 = 14;
const KEY_DOWN: u8 = 15;
const KEY_AB: u8 = 17;

const BLACK: u16 = 0x0000;
const WHITE: u16 = 0xFFFF;
const GRAY: u16 = 0x8410;
const GREEN: u16 = 0x07E0;

const F6: u8 = 1; // 6x10
const F9: u8 = 2; // 9x18

const SCREEN_W: i16 = 160;

const TITLE_Y: i16 = 4;
const FREQ_CLEAR_Y: i16 = 26;
const FREQ_CLEAR_H: i16 = 22;
const FREQ_Y: i16 = 30;
const STATUS_CLEAR_Y: i16 = 58;
const STATUS_CLEAR_H: i16 = 14;
const STATUS_Y: i16 = 61;
const TONE_X: i16 = 90;
const HINT_Y: i16 = 111;

const TONE_TIMEOUT_100US: u32 = 5000;

/// Two consecutive raw counter readings this close (in raw deci-Hz words)
/// are trusted as one candidate frequency, rather than two different
/// signals.
const SAMPLE_AGREEMENT_MAX: u32 = 25;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Band {
    Uhf,
    Vhf,
    Band200M,
    Band350M,
}

impl Band {
    fn next(self) -> Self {
        match self {
            Band::Uhf => Band::Vhf,
            Band::Vhf => Band::Band200M,
            Band::Band200M => Band::Band350M,
            Band::Band350M => Band::Uhf,
        }
    }

    fn uhf_path(self) -> bool {
        matches!(self, Band::Uhf | Band::Band350M)
    }

    fn label(self) -> &'static str {
        match self {
            Band::Uhf => "UHF",
            Band::Vhf => "VHF",
            Band::Band200M => "220M",
            Band::Band350M => "350M",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Setup,
    Hunt,
    Check,
    ToneSetup,
    ToneWait,
    Found,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Status {
    Hunting,
    Listening,
    Found,
}

impl Phase {
    fn status(self) -> Status {
        match self {
            Phase::Setup | Phase::Hunt | Phase::Check => Status::Hunting,
            Phase::ToneSetup | Phase::ToneWait => Status::Listening,
            Phase::Found => Status::Found,
        }
    }
}

struct State {
    band: Band,
    phase: Phase,
    /// First raw counter sample this hunt cycle, waiting for a second one
    /// to agree with. `None` = none taken yet.
    sample: Option<u32>,
    /// Candidate frequency, in raw deci-Hz words.
    found_freq_word: u32,
    timeout_100us: u32,
    /// Merged subaudio index once a standard tone was decoded; `-1` = none
    /// (still hunting a tone, or gave up without a standard match).
    tone_index: i32,
}

static mut STATE: State = State {
    band: Band::Uhf,
    phase: Phase::Setup,
    sample: None,
    found_freq_word: 0,
    timeout_100us: 0,
    tone_index: -1,
};

static mut API_PTR: *const Api = core::ptr::null();

#[derive(Clone, Copy, PartialEq, Eq)]
struct Snap {
    band: Band,
    phase: Phase,
    found_freq_word: u32,
    tone_index: i32,
}

static mut LAST_SNAP: Option<Snap> = None;

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
        AppEvent::Leave => AppResult::Continue,
    }
}

fn correct_band_fold(band: Band, freq: u32) -> Option<u32> {
    let mut freq = freq;
    match band {
        Band::Uhf => {
            if freq > 52_000_000 {
                freq /= 2;
            }
            if freq < 40_000_000 {
                return None;
            }
        }
        Band::Vhf => {
            if freq >= 36_000_000 {
                freq /= 3;
            } else if freq > 25_000_000 {
                freq /= 2;
            } else if freq < 10_000_000 {
                return None;
            }
            if freq > 20_000_000 {
                return None;
            }
        }
        Band::Band200M => {
            if freq >= 40_000_000 {
                freq /= 2;
            } else if freq < 20_000_000 {
                return None;
            }
            if freq > 30_000_000 {
                return None;
            }
        }
        Band::Band350M => {
            if freq >= 70_000_000 {
                freq /= 2;
            } else if freq < 30_000_000 {
                return None;
            }
            if freq > 40_000_000 {
                return None;
            }
        }
    }
    Some(freq)
}

fn clamp_band(band: Band, freq: u32) -> u32 {
    match band {
        Band::Vhf => freq.clamp(13_600_000, 17_400_000),
        Band::Band200M => freq.clamp(20_000_000, 26_000_000),
        Band::Band350M => {
            if freq > 39_000_000 {
                39_000_000
            } else if freq < 30_000_000 {
                35_000_000
            } else {
                freq
            }
        }
        Band::Uhf => freq.clamp(40_000_000, 52_000_000),
    }
}

fn enter(api: &Api) -> AppResult {
    // Overlay entry parks the transceiver (`Radio::stop_for_app`); the
    // in-flash version never touched RF setup here because standby had
    // already left the chip live. Wake it back into whatever the master
    // side was watching before hunting starts.
    let freq = (api.get_master_freq)();
    let wide = (api.get_master_wide)();
    (api.set_modulation)(0);
    (api.set_rx_freq)(freq);
    (api.set_bandwidth)(wide);
    (api.set_subaudio_rx)(0);
    (api.set_sql_level)((api.settings_get)(0) as u8);
    (api.enter_rx)();

    unsafe {
        STATE = State {
            band: Band::Uhf,
            phase: Phase::Setup,
            sample: None,
            found_freq_word: 0,
            timeout_100us: 0,
            tone_index: -1,
        };
        LAST_SNAP = None;
    }
    AppResult::Continue
}

fn leave(api: &Api) -> AppResult {
    (api.freq_scan_disable)();
    (api.set_subaudio_scan_filter)(false);
    AppResult::Exit
}

fn save(api: &Api) -> AppResult {
    let st = unsafe { &STATE };
    let freq_hz = st.found_freq_word * 10;
    let code = if st.tone_index >= 0 {
        (api.subaudio_code_of_index)(st.tone_index)
    } else {
        0
    };
    (api.save_master_vfo)(freq_hz, code);
    leave(api)
}

fn restart_hunt(st: &mut State) {
    st.sample = None;
    st.tone_index = -1;
    st.phase = Phase::Setup;
}

fn tick(api: &Api, dt_100us: u32) -> AppResult {
    let st = unsafe { &mut STATE };

    match st.phase {
        Phase::Setup => {
            (api.freq_scan_enable)();
            st.sample = None;
            st.phase = Phase::Hunt;
        }
        Phase::Hunt => {
            let raw = (api.check_freq_scan)();
            if raw == u32::MAX {
                return AppResult::Continue;
            }
            (api.freq_scan_disable)();
            match st.sample.take() {
                None => {
                    st.sample = Some(raw);
                    (api.freq_scan_enable)();
                }
                Some(first) => {
                    if first.abs_diff(raw) < SAMPLE_AGREEMENT_MAX {
                        let avg = (first + raw) / 2;
                        match correct_band_fold(st.band, avg) {
                            Some(corrected) => {
                                st.found_freq_word = corrected;
                                st.phase = Phase::Check;
                            }
                            None => st.phase = Phase::Setup,
                        }
                    } else {
                        (api.freq_scan_enable)();
                    }
                }
            }
        }
        Phase::Check => {
            let corrected =
                (api.correct_measured_freq_word)(st.found_freq_word);
            let rounded = (corrected + 13) / 25 * 25;
            let clamped = clamp_band(st.band, rounded);
            st.found_freq_word = clamped;
            (api.tune_search_candidate)(clamped * 10, st.band.uhf_path());
            (api.set_subaudio_scan_filter)(true);
            st.phase = Phase::ToneSetup;
        }
        Phase::ToneSetup => {
            st.timeout_100us = TONE_TIMEOUT_100US;
            st.tone_index = -1;
            st.phase = Phase::ToneWait;
        }
        Phase::ToneWait => {
            let idx = (api.detect_subaudio)();
            if idx == -1 {
                st.timeout_100us = st.timeout_100us.saturating_sub(dt_100us);
                if st.timeout_100us == 0 {
                    st.phase = Phase::Found;
                }
            } else if idx <= -2 {
                // Decoded something, but not a standard tone: same outcome
                // as a timeout (report no tone), no reason to keep waiting.
                st.tone_index = -1;
                st.phase = Phase::Found;
            } else {
                st.tone_index = idx;
                st.phase = Phase::Found;
            }
        }
        Phase::Found => {}
    }
    AppResult::Continue
}

fn key(api: &Api, id: u8, kind: u8) -> AppResult {
    if kind != KIND_SINGLE && kind != KIND_LONG {
        return AppResult::Continue;
    }
    let st = unsafe { &mut STATE };
    match (id, kind) {
        (KEY_AB, KIND_SINGLE) => {
            st.band = st.band.next();
            restart_hunt(st);
            AppResult::Continue
        }
        (KEY_UP, KIND_SINGLE) | (KEY_DOWN, KIND_SINGLE)
            if st.phase == Phase::Found =>
        {
            restart_hunt(st);
            AppResult::Continue
        }
        (KEY_MENU, KIND_SINGLE) if st.phase == Phase::Found => save(api),
        (KEY_EXIT, KIND_SINGLE) | (KEY_EXIT, KIND_LONG) => leave(api),
        _ => AppResult::Continue,
    }
}

// ui

fn text(api: &Api, x: i16, y: i16, s: &[u8], fg: u16, bg: u16, font: u8) {
    (api.draw_text)(x, y, s.as_ptr(), s.len() as u16, fg, bg, font);
}

fn clear_strip(api: &Api, y: i16, h: i16) {
    (api.fill_rect)(0, y, SCREEN_W as u16, h as u16, BLACK);
}

fn draw(api: &Api) -> AppResult {
    let st = unsafe { &STATE };
    let snap = Snap {
        band: st.band,
        phase: st.phase,
        found_freq_word: st.found_freq_word,
        tone_index: st.tone_index,
    };
    let prev = unsafe { LAST_SNAP };

    match prev {
        None => full_redraw(api, &snap),
        Some(p) => {
            if p.band != snap.band {
                draw_header(api, &snap);
            }
            if (p.phase, p.found_freq_word, p.tone_index)
                != (snap.phase, snap.found_freq_word, snap.tone_index)
            {
                draw_content(api, &snap);
            }
        }
    }

    unsafe { LAST_SNAP = Some(snap) };
    AppResult::Continue
}

fn full_redraw(api: &Api, snap: &Snap) {
    (api.fill_rect)(0, 0, SCREEN_W as u16, 128, BLACK);
    draw_header(api, snap);
    draw_content(api, snap);
    text(api, 4, HINT_Y, b"AB BAND  MENU SAVE", GRAY, BLACK, F6);
}

fn draw_header(api: &Api, snap: &Snap) {
    clear_strip(api, TITLE_Y, 12);
    let mut buf = [0u8; 20];
    let prefix = b"SEARCH ";
    buf[..prefix.len()].copy_from_slice(prefix);
    let label = snap.band.label().as_bytes();
    buf[prefix.len()..prefix.len() + label.len()].copy_from_slice(label);
    let n = prefix.len() + label.len();
    text(api, 4, TITLE_Y, &buf[..n], WHITE, BLACK, F6);
}

fn draw_content(api: &Api, snap: &Snap) {
    clear_strip(api, FREQ_CLEAR_Y, FREQ_CLEAR_H);
    clear_strip(api, STATUS_CLEAR_Y, STATUS_CLEAR_H);

    match snap.phase.status() {
        Status::Hunting => {
            text(api, 4, STATUS_Y, b"HUNTING...", WHITE, BLACK, F6);
        }
        Status::Listening => {
            draw_freq(api, snap.found_freq_word);
            text(api, 4, STATUS_Y, b"LISTEN", WHITE, BLACK, F6);
        }
        Status::Found => {
            draw_freq(api, snap.found_freq_word);
            text(api, 4, STATUS_Y, b"FOUND", WHITE, BLACK, F6);
            let mut buf = [0u8; 16];
            let n = (api.subaudio_format)(snap.tone_index, buf.as_mut_ptr(), 16)
                as usize;
            text(api, TONE_X, STATUS_Y, &buf[..n], GREEN, BLACK, F6);
        }
    }
}

fn draw_freq(api: &Api, found_freq_word: u32) {
    let mut buf = [0u8; 12];
    let n = (api.fmt_freq)(found_freq_word * 10, buf.as_mut_ptr()) as usize;
    text(api, 4, FREQ_Y, &buf[..n], WHITE, BLACK, F9);
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
