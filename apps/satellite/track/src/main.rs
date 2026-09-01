#![no_std]
#![no_main]

use bf5rh_abi::{Api, AppEvent, AppResult};

#[path = "../../common.rs"]
mod common;
use common::*;

static mut API_PTR: *const Api = core::ptr::null();

const GAIN_NAMES: [&[u8]; 4] = [b"LNAs", b"LNA", b"PGA", b"IF"];

struct State {
    ready: bool,
    back: Handoff,
    head: RecordHead,
    calib: CalibHeader,
    aos: u32,
    los: u32,

    /// interpolated at `last_secs`
    rate_q: i16,
    az_cdeg: u16,
    el_cdeg: i16,
    doppler_hz: i32,
    last_secs: u32,

    sql: u8,
    monitor: bool,
    wide: bool,
    transmitting: bool,

    /// 0 = off, 1..=4 select LNAs/LNA/PGA/IF
    gain_menu: u8,
    gains: [u16; 4],

    /// Repaint the chrome that never changes (title bar, hints) as well.
    full: bool,
    /// Repaint the live fields. Set once a second by `tick`, and on any key.
    dirty: bool,
    /// Projected samples held in `PLOT`.
    plot_n: u8,
    /// Peak elevation of the whole pass, for the gauge's high-water mark.
    max_el_cdeg: i16,
    /// Where the "you are here" dot was last drawn, so it can be erased.
    dot_x: i16,
    dot_y: i16,
    /// Lit segment count the S-meter currently shows.
    shown_lit: i16,
}

static mut PLOT: [u8; 2 * MAX_CALIB_SAMPLES] = [0; 2 * MAX_CALIB_SAMPLES];

static mut STATE: State = State {
    ready: false,
    back: Handoff::BLANK,
    head: RecordHead::BLANK,
    calib: CalibHeader {
        sat_index: 0,
        pass_index: 0,
        count: 0,
        t0: 0,
        aos: 0,
        los: 0,
        step_s: 1,
    },
    aos: 0,
    los: 0,
    rate_q: 0,
    az_cdeg: 0,
    el_cdeg: 0,
    doppler_hz: 0,
    last_secs: 0,
    sql: 0,
    monitor: false,
    wide: true,
    transmitting: false,
    gain_menu: 0,
    gains: [0; 4],
    full: true,
    dirty: true,
    plot_n: 0,
    max_el_cdeg: 0,
    dot_x: 0,
    dot_y: 0,
    shown_lit: -1,
};

fn state() -> &'static mut State {
    unsafe { &mut *core::ptr::addr_of_mut!(STATE) }
}

#[no_mangle]
pub extern "C" fn app_entry(api: &Api, ev: AppEvent) -> AppResult {
    unsafe {
        API_PTR = api as *const Api;
    }
    match ev {
        AppEvent::Enter => enter(api),
        AppEvent::Key { id, kind } => key(api, kind, id),
        AppEvent::Tick { .. } => tick(api),
        AppEvent::Draw => draw(api),
        AppEvent::Leave => AppResult::Continue,
    }
}

fn enter(api: &Api) -> AppResult {
    let st = state();
    st.ready = false;

    // No handoff means this segment was entered straight from the launcher,
    // which it never should be: there is no pass to track.
    let Some(h) = Handoff::load(api) else {
        return AppResult::Exit;
    };
    st.back = Handoff {
        page: PAGE_PASSES,
        ..h
    };

    // Everything below has to line up, or the tables in flash belong to some
    // other satellite and tracking them would put the radio on a wrong
    // frequency: bounce back to the list instead.
    let (Some(head), Some(calib)) =
        (read_head(api, h.sat_index as usize), read_calib_header(api))
    else {
        return bounce(api, st);
    };
    if calib.sat_index != h.sat_index || calib.pass_index != h.pass_index {
        return bounce(api, st);
    }

    st.head = head;
    st.calib = calib;
    st.aos = calib.aos;
    st.los = calib.los;
    st.sql = 0;
    st.monitor = false;
    st.wide = true;
    st.gain_menu = 0;
    st.transmitting = false;
    st.last_secs = (api.utc_get)();
    interpolate(api, st, st.last_secs);
    build_plot(api, st);
    let (dx, dy) = project(st.az_cdeg, st.el_cdeg);
    st.dot_x = dx;
    st.dot_y = dy;
    st.shown_lit = -1;
    st.full = true;
    st.dirty = true;
    st.ready = true;

    (api.set_backlight_hold)(true);

    // Program the whole RX config, then push it to the chip once; after that
    // only the frequency moves, via the cheaper retune.
    (api.set_modulation)(0); // FM
    (api.set_power)(0); // High
    (api.set_rx_freq)(head.rx_freq_hz);
    (api.set_bandwidth)(st.wide);
    (api.set_subaudio_rx)(head.rx_tone);
    (api.set_subaudio_tx)(head.tx_tone);
    (api.set_sql_level)(st.sql);
    (api.set_monitor)(false);
    (api.enter_rx)();
    apply_rx(api, st);

    // An RX-only bird has no uplink, so leave the PTT key inert
    (api.set_tx_enabled)(head.tx_freq_hz != 0);
    apply_tx_freq(api, st);
    read_gains(api, st);
    AppResult::Continue
}

fn bounce(api: &Api, st: &State) -> AppResult {
    Handoff {
        status: ST_NO_PASS,
        page: PAGE_SATS,
        ..st.back
    }
    .store(api);
    AppResult::Chain(SEG_LIST)
}

/// Linear interpolation between the two samples straddling `t`, clamped to
/// the ends of the table.
fn interpolate(api: &Api, st: &mut State, t: u32) {
    let step = st.calib.step_s as u32;
    let last = st.calib.count as u32 - 1;
    let rel = t.saturating_sub(st.calib.t0);
    let i = (rel / step).min(last - 1);
    let frac = (rel - i * step).min(step) as i32;

    let mut raw = [0u8; 2 * CALIB_SAMPLE_SIZE];
    (api.nor_read)(
        CALIB_ADDR + CALIB_HEADER_SIZE as u32 + i * CALIB_SAMPLE_SIZE as u32,
        raw.as_mut_ptr(),
        raw.len() as u32,
    );
    let a = decode_sample(&raw[..CALIB_SAMPLE_SIZE]);
    let b = decode_sample(&raw[CALIB_SAMPLE_SIZE..]);

    let lerp = |x: i32, y: i32| x + (y - x) * frac / step as i32;
    st.rate_q = lerp(a.rate_q as i32, b.rate_q as i32) as i16;
    st.el_cdeg = lerp(a.el_cdeg as i32, b.el_cdeg as i32) as i16;

    let (az0, mut az1) = (a.az_cdeg as i32, b.az_cdeg as i32);
    if az1 - az0 > 18_000 {
        az1 -= 36_000;
    } else if az0 - az1 > 18_000 {
        az1 += 36_000;
    }
    st.az_cdeg = lerp(az0, az1).rem_euclid(36_000) as u16;

    st.doppler_hz = doppler_hz(st.head.rx_freq_hz, st.rate_q);
}

fn apply_rx(api: &Api, st: &State) {
    let hz = (st.head.rx_freq_hz as i32 + st.doppler_hz).max(0) as u32;
    (api.retune_rx)(hz, st.wide);
}

fn tx_freq(st: &State) -> u32 {
    (st.head.tx_freq_hz as i32 - doppler_hz(st.head.tx_freq_hz, st.rate_q))
        .max(0) as u32
}

fn apply_tx_freq(api: &Api, st: &State) {
    if st.head.tx_freq_hz != 0 {
        (api.set_tx_freq)(tx_freq(st));
    }
}

fn read_gains(api: &Api, st: &mut State) {
    let mut buf = [0u16; 4];
    if (api.read_rf_gains)(buf.as_mut_ptr()) {
        st.gains = buf;
    }
}

fn tick(api: &Api) -> AppResult {
    let st = state();
    if !st.ready {
        return AppResult::Continue;
    }

    let tx = (api.tx_state)() & 1 != 0;
    if tx != st.transmitting {
        st.transmitting = tx;
        st.dirty = true;
    }

    let now = (api.utc_get)();
    if now == st.last_secs {
        return AppResult::Continue;
    }
    st.last_secs = now;

    let was = st.doppler_hz;
    interpolate(api, st, now);

    st.dirty = true;
    if st.doppler_hz == was {
        return AppResult::Continue;
    }

    apply_tx_freq(api, st);
    if !st.transmitting {
        apply_rx(api, st);
    }
    AppResult::Continue
}

fn key(api: &Api, kind: u8, id: u8) -> AppResult {
    if kind != KIND_SINGLE && kind != KIND_LONG && kind != KIND_REPEAT {
        return AppResult::Continue;
    }
    let st = state();
    st.dirty = true;
    match id {
        KEY_EXIT => {
            if st.gain_menu > 0 {
                close_gain_menu(st);
            } else {
                return leave(api, st);
            }
        }
        KEY_UP | KEY_DOWN => {
            let up = id == KEY_UP;
            if st.gain_menu > 0 {
                (api.adjust_rf_gain)(st.gain_menu - 1, up);
                read_gains(api, st);
            } else {
                adjust_squelch(api, st, up);
            }
        }
        KEY_MENU => {
            if st.gain_menu >= 4 {
                close_gain_menu(st);
            } else {
                st.gain_menu += 1;
            }
        }
        KEY_BAND if kind == KIND_SINGLE => {
            st.wide = !st.wide;
            (api.set_bandwidth)(st.wide);
            apply_rx(api, st);
        }
        _ => {}
    }
    AppResult::Continue
}

fn close_gain_menu(st: &mut State) {
    st.gain_menu = 0;
    st.full = true;
}

/// Squelch runs 0..9 with an extra "OFF" step below 0 that turns on monitor.
fn adjust_squelch(api: &Api, st: &mut State, up: bool) {
    if up {
        if st.monitor {
            st.monitor = false;
            st.sql = 0;
        } else {
            st.sql = (st.sql + 1).min(9);
        }
    } else if st.monitor {
        return;
    } else if st.sql == 0 {
        st.monitor = true;
    } else {
        st.sql -= 1;
    }
    (api.set_sql_level)(st.sql);
    (api.set_monitor)(st.monitor);
}

fn leave(api: &Api, st: &mut State) -> AppResult {
    st.ready = false;
    (api.set_ptt)(false);
    (api.set_tx_enabled)(false);
    (api.set_monitor)(false);
    st.back.store(api);
    AppResult::Chain(SEG_LIST)
}

// ui

const fn rgb(r: u16, g: u16, b: u16) -> u16 {
    (r << 11) | (g << 5) | b
}

const TITLE_BG: u16 = rgb(4, 8, 16);
const DIM: u16 = rgb(6, 12, 6);
const GRAY: u16 = rgb(16, 32, 16);
const CYAN: u16 = rgb(0, 48, 24);
const PAST: u16 = rgb(7, 14, 10);
const GREEN: u16 = rgb(0, 60, 0);
const YELLOW: u16 = rgb(31, 60, 0);
const ORANGE: u16 = rgb(31, 36, 0);
const RED: u16 = rgb(31, 0, 0);

const TITLE_H: u16 = 12;
const FREQ_Y: i16 = 14;
const FREQ_L: i16 = 18;
const FREQ_R: i16 = 126;
const BADGE_Y: i16 = 18;
const DOP_CHARS: usize = 6;
const DOP_X: i16 = SCREEN_W - 2 - (DOP_CHARS as i16) * 5;

const PLOT_CX: i16 = 33;
const PLOT_CY: i16 = 66;
const PLOT_R: i16 = 24;

const COL_X: i16 = 72;
const VAL_X: i16 = 88;
const OCT_X: i16 = 119;
const LAB_DY: i16 = 5;
const AZ_Y: i16 = 36;
const EL_Y: i16 = 56;
const CD_Y: i16 = 77;
const UP_Y: i16 = 89;

const GAUGE_X: i16 = 138;
const GAUGE_W: u16 = 18;
const GAUGE_Y: i16 = 34;
const GAUGE_H: i16 = 66;

const SM_Y: i16 = 102;
const SM_SEGS: i16 = 19;
const HINT_Y: i16 = 115;

fn text(api: &Api, x: i16, y: i16, s: &[u8], fg: u16, bg: u16, font: u8) {
    (api.draw_text)(x, y, s.as_ptr(), s.len() as u16, fg, bg, font);
}

fn pad(out: &mut [u8], at: usize, width: usize) -> usize {
    for slot in out.iter_mut().take(width).skip(at) {
        *slot = b' ';
    }
    width.max(at)
}

fn draw(api: &Api) -> AppResult {
    let st = state();
    if !st.ready {
        return AppResult::Continue;
    }
    if st.full {
        draw_chrome(api, st);
        st.full = false;
        st.dirty = true;
        st.shown_lit = -1;
    }
    if st.dirty {
        st.dirty = false;
        draw_status(api, st);
        draw_freq(api, st);
        if st.gain_menu > 0 {
            draw_gain_popup(api, st);
        } else {
            draw_plot(api, st);
            draw_column(api, st);
            draw_gauge(api, st);
        }
    }
    draw_smeter(api, st);
    AppResult::Continue
}

fn draw_chrome(api: &Api, st: &State) {
    (api.fill_rect)(0, 0, SCREEN_W as u16, SCREEN_H as u16, BLACK);
    (api.fill_rect)(0, 0, SCREEN_W as u16, TITLE_H, TITLE_BG);
    let n = st.head.name_len();
    if n > 0 {
        text(api, 3, 1, &st.head.name[..n], WHITE, TITLE_BG, F6);
    }
    let hint: &[u8] = b"U/D SQL   MENU GAIN   BND W/N";
    text(api, 2, HINT_Y, hint, GRAY, BLACK, F5);
}

fn draw_status(api: &Api, st: &State) {
    let mut b = [0u8; 9];
    let i = write_at(&mut b, 0, if st.wide { b"WIDE" } else { b"NARR" });
    let i = write_at(&mut b, i, b" SQL");
    if st.monitor {
        put_char(&mut b, i, b'-');
    } else {
        put_u32(&mut b, i, st.sql as u32, 0);
    }
    text(api, SCREEN_W - 56, 1, &b, WHITE, TITLE_BG, F6);
}

fn draw_freq(api: &Api, st: &State) {
    let tx = st.transmitting && st.head.tx_freq_hz != 0;
    let (hz, fg) = if tx {
        (tx_freq(st), ORANGE)
    } else {
        (
            (st.head.rx_freq_hz as i32 + st.doppler_hz).max(0) as u32,
            WHITE,
        )
    };

    let mut buf = [0u8; 12];
    let len = put_freq_hz(&mut buf, 0, hz) as i16;
    let x = FREQ_L + (FREQ_R - FREQ_L - len * 9) / 2;
    if x > FREQ_L {
        (api.fill_rect)(FREQ_L, FREQ_Y, (x - FREQ_L) as u16, 18, BLACK);
    }
    let end = x + len * 9;
    if end < FREQ_R {
        (api.fill_rect)(end, FREQ_Y, (FREQ_R - end) as u16, 18, BLACK);
    }
    text(api, x, FREQ_Y, &buf[..len as usize], fg, BLACK, F9);

    let (badge, bfg, bbg): (&[u8], u16, u16) = if tx {
        (b"TX", BLACK, RED)
    } else {
        (b"RX", GRAY, BLACK)
    };
    text(api, 2, BADGE_Y, badge, bfg, bbg, F5);

    text(api, SCREEN_W - 2 - 15, FREQ_Y, b"DOP", GRAY, BLACK, F5);
    let mut d = [0u8; DOP_CHARS];
    let i = put_char(&mut d, 0, if st.doppler_hz < 0 { b'-' } else { b'+' });
    let i = put_u32(&mut d, i, st.doppler_hz.unsigned_abs(), 0);
    right_align(&mut d, i, DOP_CHARS);
    text(api, DOP_X, FREQ_Y + 9, &d, CYAN, BLACK, F5);
}

fn draw_column(api: &Api, st: &State) {
    text(api, COL_X, AZ_Y + LAB_DY, b"AZ", GRAY, BLACK, F5);
    let mut b = [0u8; 3];
    put_u32(&mut b, 0, st.az_cdeg as u32 / 100, 3);
    text(api, VAL_X, AZ_Y, &b, WHITE, BLACK, F9);
    let mut o = [b' '; 2];
    write_at(&mut o, 0, octant(st.az_cdeg));
    text(api, OCT_X, AZ_Y + LAB_DY, &o, GRAY, BLACK, F5);

    let el = st.el_cdeg as i32 / 100;
    text(api, COL_X, EL_Y + LAB_DY, b"EL", GRAY, BLACK, F5);
    let mut b = [0u8; 3];
    let i = put_i32(&mut b, 0, el, 0);
    let i = pad(&mut b, i, 3);
    text(api, VAL_X, EL_Y, &b[..i], el_color(el), BLACK, F9);

    let now = st.last_secs;
    let mut cd = [0u8; 10];
    let (i, fg) = if now < st.aos {
        let i = write_at(&mut cd, 0, b"T-");
        (put_duration(&mut cd, i, st.aos - now), ORANGE)
    } else if now < st.los {
        let i = write_at(&mut cd, 0, b"T+");
        (put_duration(&mut cd, i, now - st.aos), GREEN)
    } else {
        (write_at(&mut cd, 0, b"PASSED"), GRAY)
    };
    let i = pad(&mut cd, i, 10);
    text(api, COL_X, CD_Y, &cd[..i], fg, BLACK, F6);

    let mut up = [0u8; 10];
    let i = if st.head.tx_freq_hz == 0 {
        write_at(&mut up, 0, b"RX ONLY")
    } else {
        let i = write_at(&mut up, 0, b"U");
        put_freq_mhz(&mut up, i, tx_freq(st))
    };
    let i = pad(&mut up, i, 10);
    text(api, COL_X, UP_Y, &up[..i], GRAY, BLACK, F6);
}

fn el_color(el_deg: i32) -> u16 {
    if el_deg < 10 {
        ORANGE
    } else if el_deg < 30 {
        YELLOW
    } else {
        GREEN
    }
}

fn draw_gauge(api: &Api, st: &State) {
    (api.fill_rect)(GAUGE_X, GAUGE_Y, GAUGE_W, GAUGE_H as u16, GRAY);
    let (x, y) = (GAUGE_X + 1, GAUGE_Y + 1);
    let (w, h) = (GAUGE_W - 2, GAUGE_H - 2);
    let scale = |cdeg: i32| (cdeg.clamp(0, 9000) * h as i32 / 9000) as i16;

    let lit = scale(st.el_cdeg as i32);
    let top = y + h - lit;
    (api.fill_rect)(x, y, w, (top - y) as u16, DIM);
    if lit > 0 {
        let c = el_color(st.el_cdeg as i32 / 100);
        (api.fill_rect)(x, top, w, lit as u16, c);
    }
    (api.fill_rect)(x, y + h - scale(3000), w, 1, GRAY);
    (api.fill_rect)(x, y + h - scale(6000), w, 1, GRAY);
    (api.fill_rect)(x, y + h - scale(st.max_el_cdeg as i32), w, 1, WHITE);
}

// sky plot

const SIN_Q: [i8; 91] = [
    0, 2, 4, 7, 9, 11, 13, 15, 18, 20, 22, 24, 26, 29, 31, 33, 35, 37, 39, 41,
    43, 46, 48, 50, 52, 54, 56, 58, 60, 62, 63, 65, 67, 69, 71, 73, 75, 76, 78,
    80, 82, 83, 85, 87, 88, 90, 91, 93, 94, 96, 97, 99, 100, 101, 103, 104,
    105, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 119,
    120, 121, 121, 122, 123, 123, 124, 124, 125, 125, 125, 126, 126, 126, 127,
    127, 127, 127, 127, 127,
];

/// `deg` is `0..450`: azimuth, plus the quarter turn that makes a cosine.
fn sin127(deg: i32) -> i32 {
    let d = if deg >= 360 { deg - 360 } else { deg };
    let (i, neg) = match d / 90 {
        0 => (d, false),
        1 => (180 - d, false),
        2 => (d - 180, true),
        _ => (360 - d, true),
    };
    let v = SIN_Q[i as usize] as i32;
    if neg {
        -v
    } else {
        v
    }
}

fn project(az_cdeg: u16, el_cdeg: i16) -> (i16, i16) {
    let el = (el_cdeg as i32).clamp(0, 9000);
    let r = ((9000 - el) * PLOT_R as i32 + 4500) / 9000;
    let a = az_cdeg as i32 / 100;
    let x = PLOT_CX as i32 + ((r * sin127(a) + 64) >> 7);
    let y = PLOT_CY as i32 - ((r * sin127(a + 90) + 64) >> 7);
    (x as i16, y as i16)
}

fn build_plot(api: &Api, st: &mut State) {
    let n = (st.calib.count as usize).min(MAX_CALIB_SAMPLES);
    let plot = unsafe { &mut *core::ptr::addr_of_mut!(PLOT) };

    const CHUNK: usize = 8;
    let mut raw = [0u8; CHUNK * CALIB_SAMPLE_SIZE];
    let mut max = 0i16;
    let mut i = 0;
    while i < n {
        let k = (n - i).min(CHUNK);
        (api.nor_read)(
            CALIB_ADDR
                + CALIB_HEADER_SIZE as u32
                + (i * CALIB_SAMPLE_SIZE) as u32,
            raw.as_mut_ptr(),
            (k * CALIB_SAMPLE_SIZE) as u32,
        );
        for j in 0..k {
            let s = decode_sample(&raw[j * CALIB_SAMPLE_SIZE..]);
            let (x, y) = project(s.az_cdeg, s.el_cdeg);
            plot[2 * (i + j)] = x as u8;
            plot[2 * (i + j) + 1] = y as u8;
            if s.el_cdeg > max {
                max = s.el_cdeg;
            }
        }
        i += k;
    }
    st.plot_n = n as u8;
    st.max_el_cdeg = max;
}

fn point(plot: &[u8], i: usize) -> (i16, i16) {
    (plot[2 * i] as i16, plot[2 * i + 1] as i16)
}

fn draw_plot(api: &Api, st: &mut State) {
    (api.fill_rect)(st.dot_x - 3, st.dot_y - 3, 7, 7, BLACK);

    (api.draw_circle)(PLOT_CX, PLOT_CY, PLOT_R as u16, GRAY, false);
    (api.draw_circle)(PLOT_CX, PLOT_CY, 16, DIM, false); // 30 deg
    (api.draw_circle)(PLOT_CX, PLOT_CY, 8, DIM, false); // 60 deg
    text(
        api,
        PLOT_CX - 2,
        PLOT_CY - PLOT_R - 9,
        b"N",
        GRAY,
        BLACK,
        F5,
    );
    text(
        api,
        PLOT_CX - 2,
        PLOT_CY + PLOT_R + 2,
        b"S",
        GRAY,
        BLACK,
        F5,
    );
    text(
        api,
        PLOT_CX + PLOT_R + 2,
        PLOT_CY - 4,
        b"E",
        GRAY,
        BLACK,
        F5,
    );
    text(
        api,
        PLOT_CX - PLOT_R - 7,
        PLOT_CY - 4,
        b"W",
        GRAY,
        BLACK,
        F5,
    );

    let plot = unsafe { &*core::ptr::addr_of!(PLOT) };
    let n = st.plot_n as usize;
    let cur = st.last_secs.saturating_sub(st.calib.t0)
        / st.calib.step_s.max(1) as u32;
    for i in 1..n {
        let (x0, y0) = point(plot, i - 1);
        let (x1, y1) = point(plot, i);
        let c = if (i as u32) <= cur { PAST } else { CYAN };
        (api.draw_line)(x0, y0, x1, y1, c);
    }
    if n > 1 {
        let (x, y) = point(plot, 0);
        (api.draw_circle)(x, y, 1, GREEN, true);
        let (x, y) = point(plot, n - 1);
        (api.draw_circle)(x, y, 1, RED, true);
    }

    let (x, y) = project(st.az_cdeg, st.el_cdeg);
    (api.draw_circle)(x, y, 3, YELLOW, true);
    st.dot_x = x;
    st.dot_y = y;
}

fn draw_smeter(api: &Api, st: &mut State) {
    let rssi = (api.read_rssi)();
    let lit = (rssi.min(320) as i32 * SM_SEGS as i32 / 320) as i16;
    if lit == st.shown_lit {
        return;
    }
    st.shown_lit = lit;
    for i in 0..SM_SEGS {
        let c = if i >= lit {
            DIM
        } else if i < 11 {
            GREEN
        } else if i < 15 {
            YELLOW
        } else {
            RED
        };
        (api.fill_rect)(2 + i * 8, SM_Y, 7, 8, c);
    }
}

fn draw_gain_popup(api: &Api, st: &State) {
    text(
        api,
        2,
        HINT_Y,
        b"U/D ADJ  MENU NEXT  EXIT CLOSE",
        GRAY,
        BLACK,
        F5,
    );
    (api.fill_rect)(2, 34, 156, 66, GRAY);
    (api.fill_rect)(3, 35, 154, 64, BLACK);
    for k in 0..4usize {
        let y = 42 + (k as i16) * 14;
        let selected = st.gain_menu == (k + 1) as u8;
        let (fg, bg) = if selected {
            (BLACK, WHITE)
        } else {
            (WHITE, BLACK)
        };
        if selected {
            (api.fill_rect)(6, y - 1, 148, 12, WHITE);
        }
        text(api, 10, y, GAIN_NAMES[k], fg, bg, F6);
        let mut v = [0u8; 3];
        let i = put_u32(&mut v, 0, st.gains[k] as u32, 0);
        text(api, 150 - (i as i16) * 6, y, &v[..i], fg, bg, F6);
    }
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
