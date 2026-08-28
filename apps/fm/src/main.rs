#![no_std]
#![no_main]

use bf5rh_abi::{Api, AppEvent, AppResult};

const KIND_SINGLE: u8 = 2;
const KIND_LONG: u8 = 3;
const KIND_REPEAT: u8 = 4;
const KEY_MENU: u8 = 12;
const KEY_EXIT: u8 = 13;
const KEY_UP: u8 = 14;
const KEY_DOWN: u8 = 15;
const KEY_VM: u8 = 16;

const FREQ_LO_DECI_MHZ: u16 = 875;
const FREQ_HI_DECI_MHZ: u16 = 1080;
const FM_DEFAULT_DECI_MHZ: u16 = 1000;

const FREQ_INPUT_DIGITS: usize = 4;
const CHANNEL_INPUT_DIGITS: usize = 2;
const CHANNEL_COUNT: usize = 30;
const CHANNEL_EMPTY: u16 = 0xFFFF;
const SEEK_TIMEOUT_TICKS: u16 = 300;

const BLACK: u16 = 0x0000;
const WHITE: u16 = 0xFFFF;
const TITLE_BG: u16 = 0x0881;

const F6: u8 = 1; // 6x10
const F9: u8 = 2; // 9x18
const FW6: i16 = 6;
const FW9: i16 = 9;

const SCREEN_W: i16 = 160;
const SCREEN_H: i16 = 128;
const RIGHT_MARGIN: i16 = 4;

const HEADER_CLEAR_Y: i16 = 4;
const HEADER_CLEAR_H: i16 = 12;
const HEADER_TOP: i16 = 5;
const FREQ_CLEAR_Y: i16 = 44;
const FREQ_CLEAR_H: i16 = 22;
const FREQ_TOP: i16 = 47; // 9x18 baseline 60
const STATUS_CLEAR_Y: i16 = 84;
const STATUS_CLEAR_H: i16 = 14;
const STATUS_TOP: i16 = 87; // 6x10 baseline 96
const HINT_TOP: i16 = 111; // 6x10 baseline 120

const TITLE_HEIGHT: i16 = 14;
const ROW_HEIGHT: i16 = 14;
const LIST_TOP: i16 = 14;
const VISIBLE_ROWS: usize = 8;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Vfo,
    Channel,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Tuning,
    Seeking { timeout: u16 },
    SavePicker { selected: u8 },
}

struct State {
    mode: Mode,
    deci_mhz: u16,
    channel_index: u8,
    phase: Phase,
    rssi: u8,
    channels: [u16; CHANNEL_COUNT],
    input: [u8; FREQ_INPUT_DIGITS],
    input_len: usize,
}

const EMPTY_CHANNELS: [u16; CHANNEL_COUNT] = [CHANNEL_EMPTY; CHANNEL_COUNT];

static mut STATE: State = State {
    mode: Mode::Vfo,
    deci_mhz: FM_DEFAULT_DECI_MHZ,
    channel_index: 0,
    phase: Phase::Tuning,
    rssi: 0,
    channels: EMPTY_CHANNELS,
    input: [0u8; FREQ_INPUT_DIGITS],
    input_len: 0,
};

static mut API_PTR: *const Api = core::ptr::null();

#[derive(Clone, Copy, PartialEq, Eq)]
enum View {
    Tuning,
    Picker,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Snap {
    view: View,
    channel_mode: bool,
    channel_index: u8,
    input_len: usize,
    input_digits: [u8; FREQ_INPUT_DIGITS],
    deci_mhz: u16,
    seeking: bool,
    rssi: u8,
    picker_selected: u8,
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
        AppEvent::Tick { .. } => tick(api),
        AppEvent::Draw => draw(api),
        AppEvent::Leave => AppResult::Continue,
    }
}

fn enter(api: &Api) -> AppResult {
    // park for fm
    (api.fm_tune_khz)(FM_DEFAULT_DECI_MHZ as u32 * 100);

    // read channels
    let mut ch = EMPTY_CHANNELS;
    let _ok = (api.fm_channels_load)(
        ch.as_mut_ptr() as *mut u8,
        (CHANNEL_COUNT * 2) as u16,
    );

    // init state
    unsafe {
        STATE = State {
            mode: Mode::Vfo,
            deci_mhz: FM_DEFAULT_DECI_MHZ,
            channel_index: 0,
            phase: Phase::Tuning,
            rssi: 0,
            channels: ch,
            input: [0u8; FREQ_INPUT_DIGITS],
            input_len: 0,
        };
        LAST_SNAP = None;
    }

    // enable speaker
    (api.set_speaker)(true);
    AppResult::Continue
}

fn exit(api: &Api) {
    (api.fm_power_off)();
}

fn tune(api: &Api, deci_mhz: u16) {
    unsafe {
        STATE.deci_mhz = deci_mhz;
    }
    (api.fm_tune_khz)(deci_mhz as u32 * 100);
}

fn channels() -> [u16; CHANNEL_COUNT] {
    unsafe { STATE.channels }
}

fn input_value() -> u16 {
    let len = unsafe { STATE.input_len };
    let mut v = 0u16;
    for i in 0..len {
        v = v * 10 + unsafe { STATE.input[i] as u16 };
    }
    v
}

fn find_next_channel(
    channels: &[u16; CHANNEL_COUNT],
    from: u8,
    forward: bool,
) -> Option<u8> {
    let len = CHANNEL_COUNT as u8;
    let mut i = from;
    for _ in 0..len {
        i = if forward {
            if i + 1 >= len {
                0
            } else {
                i + 1
            }
        } else if i == 0 {
            len - 1
        } else {
            i - 1
        };
        if channels[i as usize] != CHANNEL_EMPTY {
            return Some(i);
        }
    }
    None
}

fn find_matching_or_first(
    channels: &[u16; CHANNEL_COUNT],
    deci_mhz: u16,
) -> Option<u8> {
    channels
        .iter()
        .position(|&f| f == deci_mhz)
        .or_else(|| channels.iter().position(|&f| f != CHANNEL_EMPTY))
        .map(|i| i as u8)
}

fn step(api: &Api, up: bool) {
    let mode = unsafe { STATE.mode };
    match mode {
        Mode::Vfo => {
            let cur = unsafe { STATE.deci_mhz };
            let next = if up {
                if cur >= FREQ_HI_DECI_MHZ {
                    FREQ_LO_DECI_MHZ
                } else {
                    cur + 1
                }
            } else if cur <= FREQ_LO_DECI_MHZ {
                FREQ_HI_DECI_MHZ
            } else {
                cur - 1
            };
            tune(api, next);
        }
        Mode::Channel => {
            let ch = channels();
            if let Some(idx) =
                find_next_channel(&ch, unsafe { STATE.channel_index }, up)
            {
                unsafe {
                    STATE.channel_index = idx;
                }
                tune(api, ch[idx as usize]);
            }
        }
    }
}

fn start_seek(api: &Api, up: bool) {
    (api.set_speaker)(false);
    (api.fm_seek)(up);
    unsafe {
        STATE.phase = Phase::Seeking {
            timeout: SEEK_TIMEOUT_TICKS,
        };
    }
}

fn toggle_mode(api: &Api) {
    let mode = unsafe { STATE.mode };
    match mode {
        Mode::Vfo => {
            match find_matching_or_first(&channels(), unsafe { STATE.deci_mhz })
            {
                Some(idx) => {
                    unsafe {
                        STATE.mode = Mode::Channel;
                        STATE.channel_index = idx;
                    }
                    tune(api, channels()[idx as usize]);
                }
                None => {
                    // TODO: play beep
                }
            }
        }
        Mode::Channel => unsafe {
            STATE.mode = Mode::Vfo;
        },
    }
    unsafe {
        STATE.input_len = 0;
    }
}

fn handle_digit(api: &Api, d: u8) {
    let mode = unsafe { STATE.mode };
    let max_len = if mode == Mode::Vfo {
        FREQ_INPUT_DIGITS
    } else {
        CHANNEL_INPUT_DIGITS
    };
    if unsafe { STATE.input_len } >= max_len {
        unsafe {
            STATE.input_len = 0;
        }
    }
    let len = unsafe { STATE.input_len };
    unsafe {
        STATE.input[len] = d;
        STATE.input_len = len + 1;
    }
    if unsafe { STATE.input_len } == max_len {
        commit_input(api);
    }
}

fn commit_input(api: &Api) {
    let value = input_value();
    unsafe {
        STATE.input_len = 0;
    }
    let mode = unsafe { STATE.mode };
    match mode {
        Mode::Vfo => {
            let deci = value;
            if (FREQ_LO_DECI_MHZ..=FREQ_HI_DECI_MHZ).contains(&deci) {
                tune(api, deci);
            }
            // out of bounds
            // TODO: play beep
        }
        Mode::Channel => {
            let slot = value as usize;
            let ch = channels();
            if (1..=CHANNEL_COUNT).contains(&slot)
                && ch[slot - 1] != CHANNEL_EMPTY
            {
                unsafe {
                    STATE.channel_index = (slot - 1) as u8;
                }
                tune(api, ch[slot - 1]);
            }
        }
    }
}

fn open_save_picker() {
    let selected = match unsafe { STATE.mode } {
        Mode::Channel => unsafe { STATE.channel_index },
        Mode::Vfo => channels()
            .iter()
            .position(|&f| f == unsafe { STATE.deci_mhz })
            .map(|i| i as u8)
            .unwrap_or(0),
    };
    unsafe {
        STATE.phase = Phase::SavePicker { selected };
    }
}

fn dispatch_save_picker(api: &Api, id: u8, kind: u8) {
    let selected = match unsafe { STATE.phase } {
        Phase::SavePicker { selected } => selected,
        _ => return,
    };
    let len = CHANNEL_COUNT as u8;
    match (kind, id) {
        (KIND_SINGLE, KEY_UP) | (KIND_REPEAT, KEY_UP) => {
            let next = if selected == 0 { len - 1 } else { selected - 1 };
            unsafe {
                STATE.phase = Phase::SavePicker { selected: next };
            }
        }
        (KIND_SINGLE, KEY_DOWN) | (KIND_REPEAT, KEY_DOWN) => {
            let next = if selected + 1 >= len { 0 } else { selected + 1 };
            unsafe {
                STATE.phase = Phase::SavePicker { selected: next };
            }
        }
        (KIND_SINGLE, KEY_MENU) => {
            let mut ch = channels();
            ch[selected as usize] = unsafe { STATE.deci_mhz };
            (api.fm_channels_save)(
                ch.as_ptr() as *const u8,
                (CHANNEL_COUNT * 2) as u16,
            );
            unsafe {
                STATE.channels = ch;
                STATE.phase = Phase::Tuning;
            }
        }
        (KIND_SINGLE, KEY_EXIT) | (KIND_LONG, KEY_EXIT) => unsafe {
            STATE.phase = Phase::Tuning;
        },
        _ => {}
    }
}

fn key(api: &Api, id: u8, kind: u8) -> AppResult {
    let phase = unsafe { STATE.phase };
    match phase {
        Phase::SavePicker { .. } => {
            dispatch_save_picker(api, id, kind);
            AppResult::Continue
        }
        _ => {
            if (kind == KIND_SINGLE || kind == KIND_LONG) && id == KEY_EXIT {
                if unsafe { STATE.input_len } > 0 {
                    unsafe {
                        STATE.input_len = 0;
                    }
                } else {
                    exit(api);
                    return AppResult::Exit;
                }
                return AppResult::Continue;
            }
            if phase == Phase::Tuning {
                if kind == KIND_SINGLE && id <= 9 {
                    handle_digit(api, id);
                } else {
                    match (kind, id) {
                        (KIND_SINGLE, KEY_VM) => toggle_mode(api),
                        (KIND_SINGLE, KEY_UP) | (KIND_REPEAT, KEY_UP) => {
                            step(api, true)
                        }
                        (KIND_SINGLE, KEY_DOWN) | (KIND_REPEAT, KEY_DOWN) => {
                            step(api, false)
                        }
                        (KIND_LONG, KEY_UP) => start_seek(api, true),
                        (KIND_LONG, KEY_DOWN) => start_seek(api, false),
                        (KIND_SINGLE, KEY_MENU) => open_save_picker(),
                        _ => {}
                    }
                }
            }
            if unsafe { STATE.phase } == Phase::Tuning {
                (api.set_speaker)(true);
            }
            AppResult::Continue
        }
    }
}

fn tick(api: &Api) -> AppResult {
    let status = (api.fm_status)();
    let rssi = (status >> 8) as u8;
    unsafe {
        STATE.rssi = rssi;
    }
    if let Phase::Seeking { timeout } = unsafe { STATE.phase } {
        let complete = status & 1 != 0;
        let seek_failed = status & 2 != 0;
        if !complete && !seek_failed && timeout > 0 {
            unsafe {
                STATE.phase = Phase::Seeking {
                    timeout: timeout - 1,
                };
            }
        } else {
            if !seek_failed {
                let khz = (api.fm_tuned_freq_khz)();
                let deci = ((khz / 100) as u16)
                    .clamp(FREQ_LO_DECI_MHZ, FREQ_HI_DECI_MHZ);
                unsafe {
                    STATE.deci_mhz = deci;
                }
            }
            unsafe {
                STATE.phase = Phase::Tuning;
            }
            (api.set_speaker)(true);
        }
    }
    AppResult::Continue
}

// UI

fn clear_strip(api: &Api, y: i16, h: i16) {
    (api.fill_rect)(0, y, SCREEN_W as u16, h as u16, BLACK);
}

fn text(api: &Api, x: i16, y: i16, s: &[u8], fg: u16, bg: u16, font: u8) {
    (api.draw_text)(x, y, s.as_ptr(), s.len() as u16, fg, bg, font);
}

fn fmt_deci(api: &Api, deci: u16, out: &mut [u8; 8]) -> usize {
    let mhz = (deci / 10) as u32;
    let tenth = (deci % 10) as u32;
    let n1 = (api.fmt_u32)(mhz, out.as_mut_ptr()) as usize;
    out[n1] = b'.';
    let n2 =
        (api.fmt_u32)(tenth, unsafe { out.as_mut_ptr().add(n1 + 1) }) as usize;
    n1 + 1 + n2
}

fn capture() -> Snap {
    unsafe {
        let (view, picker_selected) = match STATE.phase {
            Phase::SavePicker { selected } => (View::Picker, selected),
            _ => (View::Tuning, 0),
        };
        Snap {
            view,
            channel_mode: STATE.mode == Mode::Channel,
            channel_index: STATE.channel_index,
            input_len: STATE.input_len,
            input_digits: STATE.input,
            deci_mhz: STATE.deci_mhz,
            seeking: matches!(STATE.phase, Phase::Seeking { .. }),
            rssi: STATE.rssi,
            picker_selected,
        }
    }
}

fn draw(api: &Api) -> AppResult {
    let snap = capture();
    let prev = unsafe { LAST_SNAP };
    let first = prev.is_none();

    match snap.view {
        View::Tuning => {
            if first || prev.unwrap().view != View::Tuning {
                full_redraw_tuning(api, &snap);
            } else {
                let p = prev.unwrap();
                if (p.channel_mode, p.channel_index)
                    != (snap.channel_mode, snap.channel_index)
                {
                    draw_header(api, &snap);
                }
                if p.channel_mode != snap.channel_mode
                    || p.input_len != snap.input_len
                    || p.input_digits != snap.input_digits
                    || p.deci_mhz != snap.deci_mhz
                {
                    clear_strip(api, FREQ_CLEAR_Y, FREQ_CLEAR_H);
                    draw_freq(api, &snap);
                }
                if (p.seeking, p.rssi) != (snap.seeking, snap.rssi) {
                    draw_status(api, &snap);
                }
            }
        }
        View::Picker => {
            if first || prev.unwrap().view != View::Picker {
                full_redraw_picker(api, &snap);
            } else if prev.unwrap().picker_selected != snap.picker_selected {
                draw_picker(api, &snap);
            }
        }
    }

    unsafe {
        LAST_SNAP = Some(snap);
    }
    AppResult::Continue
}

fn full_redraw_tuning(api: &Api, snap: &Snap) {
    clear_strip(api, 0, SCREEN_H);
    draw_header(api, snap);
    draw_freq(api, snap);
    draw_status(api, snap);
    text(api, 4, HINT_TOP, b"VM CH/VFO  MENU SAVE", WHITE, BLACK, F6);
}

fn draw_header(api: &Api, snap: &Snap) {
    clear_strip(api, HEADER_CLEAR_Y, HEADER_CLEAR_H);
    let mut buf = [0u8; 8];
    let n = if snap.channel_mode {
        let idx = snap.channel_index as usize + 1;
        buf[0] = b'F';
        buf[1] = b'M';
        buf[2] = b' ';
        buf[3] = b'C';
        buf[4] = b'H';
        buf[5] = b' ';
        buf[6] = b'0' + (idx / 10) as u8;
        buf[7] = b'0' + (idx % 10) as u8;
        8
    } else {
        buf[0] = b'F';
        buf[1] = b'M';
        buf[2] = b' ';
        buf[3] = b'V';
        buf[4] = b'F';
        buf[5] = b'O';
        6
    };
    text(api, 4, HEADER_TOP, &buf[..n], WHITE, BLACK, F6);
}

fn draw_freq(api: &Api, snap: &Snap) {
    if snap.input_len > 0 {
        draw_freq_input(api, snap);
    } else {
        let mut buf = [0u8; 8];
        let n = fmt_deci(api, snap.deci_mhz, &mut buf);
        let w = n as i16 * FW9;
        let x = SCREEN_W - RIGHT_MARGIN - w;
        text(api, x, FREQ_TOP, &buf[..n], WHITE, BLACK, F9);
    }
}

fn draw_freq_input(api: &Api, snap: &Snap) {
    let mut buf = [0u8; 8];
    let mut n = 0usize;
    let len = snap.input_len;
    if snap.channel_mode {
        for pos in 0..CHANNEL_INPUT_DIGITS {
            buf[n] = if pos < len {
                b'0' + snap.input_digits[pos]
            } else {
                b'-'
            };
            n += 1;
        }
    } else {
        for pos in 0..FREQ_INPUT_DIGITS {
            if pos == 3 {
                buf[n] = b'.';
                n += 1;
            }
            buf[n] = if pos < len {
                b'0' + snap.input_digits[pos]
            } else {
                b'-'
            };
            n += 1;
        }
    }
    let w = n as i16 * FW9;
    let x = SCREEN_W - RIGHT_MARGIN - w;
    text(api, x, FREQ_TOP, &buf[..n], WHITE, BLACK, F9);
}

fn draw_status(api: &Api, snap: &Snap) {
    clear_strip(api, STATUS_CLEAR_Y, STATUS_CLEAR_H);
    if snap.seeking {
        text(api, 4, STATUS_TOP, b"SEEK...", WHITE, BLACK, F6);
    } else {
        let mut buf = [0u8; 16];
        buf[0] = b'R';
        buf[1] = b'S';
        buf[2] = b'S';
        buf[3] = b'I';
        buf[4] = b' ';
        let n =
            (api.fmt_u32)(snap.rssi as u32, unsafe { buf.as_mut_ptr().add(5) })
                as usize;
        text(api, 4, STATUS_TOP, &buf[..5 + n], WHITE, BLACK, F6);
    }
}

fn full_redraw_picker(api: &Api, snap: &Snap) {
    clear_strip(api, 0, SCREEN_H);
    draw_picker(api, snap);
}

fn draw_picker(api: &Api, snap: &Snap) {
    // 标题栏
    (api.fill_rect)(0, 0, SCREEN_W as u16, TITLE_HEIGHT as u16, TITLE_BG);
    let title = b"SAVE TO";
    let tw = title.len() as i16 * FW6;
    text(api, (SCREEN_W - tw) / 2, 2, title, WHITE, TITLE_BG, F6);

    let selected = snap.picker_selected as usize;
    let top = scroll_top(selected);
    for slot in 0..VISIBLE_ROWS {
        let index = top + slot;
        let row_top = LIST_TOP + slot as i16 * ROW_HEIGHT;
        if index < CHANNEL_COUNT {
            draw_picker_row(api, index, selected, row_top);
        } else {
            (api.fill_rect)(
                0,
                row_top,
                SCREEN_W as u16,
                ROW_HEIGHT as u16,
                BLACK,
            );
        }
    }
}

fn scroll_top(selected: usize) -> usize {
    if CHANNEL_COUNT <= VISIBLE_ROWS {
        return 0;
    }
    let half = VISIBLE_ROWS / 2;
    selected
        .saturating_sub(half)
        .min(CHANNEL_COUNT - VISIBLE_ROWS)
}

fn draw_picker_row(api: &Api, index: usize, selected: usize, row_top: i16) {
    let is_sel = index == selected;
    let bg = if is_sel { WHITE } else { BLACK };
    let fg = if is_sel { BLACK } else { WHITE };
    (api.fill_rect)(0, row_top, SCREEN_W as u16, ROW_HEIGHT as u16, bg);
    let text_top = row_top + 2;

    let mut buf = [0u8; 8];
    let n = index as usize + 1;
    buf[0] = b'C';
    buf[1] = b'H';
    buf[2] = b' ';
    buf[3] = b'0' + (n / 10) as u8;
    buf[4] = b'0' + (n % 10) as u8;
    text(api, 4, text_top, &buf[..5], fg, bg, F6);

    let deci = channels()[index];
    if deci == CHANNEL_EMPTY {
        text(
            api,
            SCREEN_W - RIGHT_MARGIN - 5 * FW6,
            text_top,
            b"EMPTY",
            fg,
            bg,
            F6,
        );
    } else {
        let m = fmt_deci(api, deci, &mut buf);
        let w = m as i16 * FW6;
        text(
            api,
            SCREEN_W - RIGHT_MARGIN - w,
            text_top,
            &buf[..m],
            fg,
            bg,
            F6,
        );
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
