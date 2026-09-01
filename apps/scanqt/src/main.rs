#![no_std]
#![no_main]

use bf5rh_abi::{Api, AppEvent, AppResult};

const KIND_SINGLE: u8 = 2;
const KEY_MENU: u8 = 12;
const KEY_EXIT: u8 = 13;
const KEY_UP: u8 = 14;
const KEY_DOWN: u8 = 15;

const BLACK: u16 = 0x0000;
const WHITE: u16 = 0xFFFF;
const GRAY: u16 = 0x8410;
const GREEN: u16 = 0x07E0;

const F6: u8 = 1; // 6x10
const F9: u8 = 2; // 9x18

const SCREEN_W: i16 = 160;
const SCREEN_H: i16 = 128;

const TITLE_Y: i16 = 4;
const FREQ_Y: i16 = 30;
const STATUS_CLEAR_Y: i16 = 58;
const STATUS_CLEAR_H: i16 = 14;
const STATUS_Y: i16 = 61;
const TONE_X: i16 = 90;
const HINT_Y: i16 = 111;

/// `subaudio_index_of_code`'s merged index space packs `CTCSS_TABLE`
/// first (50 entries), DCS after: this is `CTCSS_TABLE.len()`, fixed
/// alongside the table itself, same constant chanmgr already hardcodes as part
/// of `SUBAUDIO_MAX_INDEX`.
const CTCSS_COUNT: i32 = 50;

/// A single successful decode is trusted immediately for DCS; CTCSS needs
/// this many consecutive ticks agreeing on the same candidate, since a CTCSS
/// decode is far more prone to a one-off false read.
const CTCSS_HIT_COUNT: u8 = 2;

const SQUELCH_DEBOUNCE_100US: u32 = 1000;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    WaitSquelch,
    Detecting,
    Found,
}

struct State {
    phase: Phase,
    debounce_100us: u32,
    /// Merged subaudio index last seen in `Detecting`, waiting for a repeat
    /// to confirm a CTCSS candidate. `-1` = none.
    candidate: i32,
    hit_count: u8,
    /// Valid once `phase == Found`.
    tone_index: i32,
}

static mut STATE: State = State {
    phase: Phase::WaitSquelch,
    debounce_100us: 0,
    candidate: -1,
    hit_count: 0,
    tone_index: -1,
};

static mut API_PTR: *const Api = core::ptr::null();

#[derive(Clone, Copy, PartialEq, Eq)]
struct Snap {
    phase: Phase,
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

fn enter(api: &Api) -> AppResult {
    // Continue watching whatever the launcher was tuned to: this app hunts
    // a tone on the current channel, it doesn't pick a new one.
    let freq = (api.get_master_freq)();
    let wide = (api.get_master_wide)();
    // only on FM
    (api.set_modulation)(0);
    (api.set_rx_freq)(freq);
    (api.set_bandwidth)(wide);
    // Hunting a tone, not filtering by one already known.
    (api.set_subaudio_rx)(0);
    (api.set_sql_level)((api.settings_get)(0) as u8);
    (api.enter_rx)();
    (api.set_subaudio_scan_filter)(false);

    unsafe {
        STATE = State {
            phase: Phase::WaitSquelch,
            debounce_100us: 0,
            candidate: -1,
            hit_count: 0,
            tone_index: -1,
        };
        LAST_SNAP = None;
    }
    AppResult::Continue
}

fn leave(api: &Api) -> AppResult {
    (api.set_subaudio_scan_filter)(false);
    AppResult::Exit
}

fn save(api: &Api) -> AppResult {
    let tone_index = unsafe { STATE.tone_index };
    if tone_index >= 0 {
        let code = (api.subaudio_code_of_index)(tone_index);
        // CTCSS writes both RX and TX; standard DCS only writes RX
        let also_tx = tone_index < CTCSS_COUNT;
        (api.save_master_subaudio)(code, also_tx);
    }
    leave(api)
}

fn tick(api: &Api, dt_100us: u32) -> AppResult {
    let squelch_open = (api.squelch_open)();
    let st = unsafe { &mut STATE };

    match st.phase {
        Phase::WaitSquelch => {
            if squelch_open {
                st.debounce_100us += dt_100us;
                if st.debounce_100us >= SQUELCH_DEBOUNCE_100US {
                    st.debounce_100us = 0;
                    st.hit_count = 0;
                    st.candidate = -1;
                    st.phase = Phase::Detecting;
                    (api.set_subaudio_scan_filter)(true);
                }
            } else {
                st.debounce_100us = 0;
            }
        }
        Phase::Detecting => {
            if !squelch_open {
                st.debounce_100us += dt_100us;
                if st.debounce_100us >= SQUELCH_DEBOUNCE_100US {
                    st.debounce_100us = 0;
                    st.phase = Phase::WaitSquelch;
                    (api.set_subaudio_scan_filter)(false);
                }
                return AppResult::Continue;
            }
            st.debounce_100us = 0;

            let idx = (api.detect_subaudio)();
            if idx < 0 {
                st.candidate = -1;
                st.hit_count = 0;
            } else if idx < CTCSS_COUNT {
                if st.candidate == idx {
                    st.hit_count += 1;
                    if st.hit_count >= CTCSS_HIT_COUNT {
                        st.tone_index = idx;
                        st.phase = Phase::Found;
                    }
                } else {
                    st.candidate = idx;
                    st.hit_count = 1;
                }
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
    if kind != KIND_SINGLE {
        return AppResult::Continue;
    }
    let st = unsafe { &mut STATE };
    match id {
        KEY_UP | KEY_DOWN if st.phase == Phase::Found => {
            st.tone_index = -1;
            st.candidate = -1;
            st.hit_count = 0;
            st.debounce_100us = 0;
            st.phase = Phase::WaitSquelch;
            (api.set_subaudio_scan_filter)(false);
            AppResult::Continue
        }
        KEY_MENU if st.phase == Phase::Found => save(api),
        KEY_EXIT => leave(api),
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
        phase: st.phase,
        tone_index: st.tone_index,
    };
    let prev = unsafe { LAST_SNAP };

    if prev.is_none() {
        full_redraw(api);
    } else if prev != Some(snap) {
        clear_strip(api, STATUS_CLEAR_Y, STATUS_CLEAR_H);
        draw_status(api, &snap);
    }

    unsafe { LAST_SNAP = Some(snap) };
    AppResult::Continue
}

fn full_redraw(api: &Api) {
    (api.fill_rect)(0, 0, SCREEN_W as u16, SCREEN_H as u16, BLACK);
    text(api, 4, TITLE_Y, b"QT SCAN", WHITE, BLACK, F6);

    let freq = (api.get_master_freq)();
    let mut buf = [0u8; 12];
    let n = (api.fmt_freq)(freq, buf.as_mut_ptr()) as usize;
    text(api, 4, FREQ_Y, &buf[..n], WHITE, BLACK, F9);

    let st = unsafe { &STATE };
    draw_status(
        api,
        &Snap {
            phase: st.phase,
            tone_index: st.tone_index,
        },
    );

    text(api, 4, HINT_Y, b"MENU SAVE  EXIT", GRAY, BLACK, F6);
}

fn draw_status(api: &Api, snap: &Snap) {
    let status: &[u8] = match snap.phase {
        Phase::WaitSquelch => b"WAITING...",
        Phase::Detecting => b"DETECTING...",
        Phase::Found => b"FOUND",
    };
    text(api, 4, STATUS_Y, status, WHITE, BLACK, F6);

    if snap.phase == Phase::Found {
        let mut buf = [0u8; 16];
        let n = (api.subaudio_format)(snap.tone_index, buf.as_mut_ptr(), 16)
            as usize;
        text(api, TONE_X, STATUS_Y, &buf[..n], GREEN, BLACK, F6);
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
