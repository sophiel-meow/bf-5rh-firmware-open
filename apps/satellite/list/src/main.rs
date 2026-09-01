#![no_std]
#![no_main]

use bf5rh_abi::{Api, AppEvent, AppResult, ListRow};

#[path = "../../common.rs"]
mod common;
use common::*;

static mut API_PTR: *const Api = core::ptr::null();

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Utc,
    Passes,
}

/// What the next `Draw` should chain into once it has painted the banner.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Pending {
    None,
    Search,
    Calibrate,
}

/// Six-digit accumulator, one type rather than a const-generic family.
#[derive(Clone, Copy)]
struct DigitInput {
    digits: [u8; 6],
    len: u8,
}

impl DigitInput {
    const fn new() -> Self {
        DigitInput {
            digits: [0; 6],
            len: 0,
        }
    }
    fn clear(&mut self) {
        self.len = 0;
    }
    fn is_empty(&self) -> bool {
        self.len == 0
    }
    fn is_full(&self) -> bool {
        self.len == 6
    }
    fn push(&mut self, digit: u8) {
        if let Some(slot) = self.digits.get_mut(self.len as usize) {
            *slot = digit;
            self.len += 1;
        }
    }
    fn backspace(&mut self) {
        self.len = self.len.saturating_sub(1);
    }
    /// The three two-digit fields, zero for anything not typed yet.
    fn pairs(&self) -> (u32, u32, u32) {
        let d = |i: usize| {
            if i < self.len as usize {
                self.digits[i] as u32
            } else {
                0
            }
        };
        (d(0) * 10 + d(1), d(2) * 10 + d(3), d(4) * 10 + d(5))
    }
    fn fill(&mut self, a: u32, b: u32, c: u32) {
        let v = [a / 10, a % 10, b / 10, b % 10, c / 10, c % 10];
        for (slot, x) in self.digits.iter_mut().zip(v) {
            *slot = x as u8;
        }
        self.len = 6;
    }
}

struct State {
    page: Page,
    date: DigitInput,
    time: DigitInput,
    /// 0 = date, 1 = time
    field: u8,
    /// days since 2000 as last saved to flash, or -1
    saved_date: i32,

    /// bit i set = slot i holds a readable satellite record
    occupancy: u32,
    /// total passes across every satellite
    pass_count: u8,
    /// selected row, an index into `ORDER`
    pass_row: usize,
    tle_age: u8,

    status: u8,
    pending: Pending,
}

static mut STATE: State = State {
    page: Page::Passes,
    date: DigitInput::new(),
    time: DigitInput::new(),
    field: 0,
    saved_date: -1,
    occupancy: 0,
    pass_count: 0,
    pass_row: 0,
    tle_age: 0,
    status: ST_OK,
    pending: Pending::None,
};

/// The combined pass table, read back from flash and sorted by AOS in
/// [`ORDER`]. These live in `.bss` rather than on the stack
static mut PASSES: [PassEntry; MAX_TOTAL_PASSES] =
    [PassEntry::BLANK; MAX_TOTAL_PASSES];
/// Indices into [`PASSES`], ascending by AOS.
static mut ORDER: [u8; MAX_TOTAL_PASSES] = [0; MAX_TOTAL_PASSES];

fn state() -> &'static mut State {
    unsafe { &mut *core::ptr::addr_of_mut!(STATE) }
}

/// The entry the selected row points at.
fn selected_pass(st: &State) -> Option<PassEntry> {
    if st.pass_count == 0 {
        return None;
    }
    let buf = unsafe { &*core::ptr::addr_of!(PASSES) };
    let ord = unsafe { &*core::ptr::addr_of!(ORDER) };
    let row = st.pass_row.min(st.pass_count as usize - 1);
    Some(buf[ord[row] as usize])
}

/// Reads the combined table from flash and sorts it by AOS into [`ORDER`],
/// returning the total pass count.
fn load_passes(api: &Api) -> u8 {
    let count = read_pass_count(api).unwrap_or(0);

    let buf = unsafe { &mut *core::ptr::addr_of_mut!(PASSES) };
    for slot in 0..count as usize {
        let mut raw = [0u8; PASS_ENTRY_SIZE];
        if read_pass_entry(api, slot, &mut raw) {
            buf[slot] = decode_pass_entry(&raw);
        } else {
            buf[slot] = PassEntry::BLANK;
        }
    }

    let ord = unsafe { &mut *core::ptr::addr_of_mut!(ORDER) };
    for i in 0..MAX_TOTAL_PASSES {
        ord[i] = i as u8;
    }

    // Insertion sort; 80 entries is small, and moving indices (not 12-byte
    // entries) keeps this from pulling in the general-purpose memcpy.
    for i in 1..count as usize {
        let key = ord[i];
        let mut j = i;
        while j > 0 && buf[ord[j - 1] as usize].aos > buf[key as usize].aos {
            ord[j] = ord[j - 1];
            j -= 1;
        }
        ord[j] = key;
    }
    count
}

#[no_mangle]
pub extern "C" fn app_entry(api: &Api, ev: AppEvent) -> AppResult {
    unsafe {
        API_PTR = api as *const Api;
    }
    match ev {
        AppEvent::Enter => enter(api),
        AppEvent::Key { id, kind } => key(api, kind, id),
        AppEvent::Draw => draw(api),
        AppEvent::Tick { .. } | AppEvent::Leave => AppResult::Continue,
    }
}

fn enter(api: &Api) -> AppResult {
    let st = state();
    st.occupancy = scan_occupancy(api);
    st.saved_date = load_saved_date(api).unwrap_or(-1);
    st.pending = Pending::None;

    (api.set_backlight_hold)(true);

    // A handoff on Enter means a sibling segment chained back here; without
    // one this is a fresh launch from the app menu.
    match Handoff::load(api) {
        Some(h) => {
            st.status = h.status;
            st.tle_age = h.tle_age_days;
            if h.status == ST_OK && h.page == PAGE_PASSES {
                st.pass_count = load_passes(api);
                st.pass_row = (h.list_row as usize)
                    .min(st.pass_count.saturating_sub(1) as usize);
                st.page = Page::Passes;
            } else if h.status == ST_NO_UTC {
                open_utc_page(st);
            } else {
                st.pass_count = 0;
                st.pass_row = 0;
                st.page = Page::Passes;
            }
        }
        None => {
            st.status = ST_OK;
            st.pass_count = 0;
            st.pass_row = 0;
            st.tle_age = 0;
            if st.occupancy == 0 {
                st.page = Page::Passes;
            } else if (api.utc_get)() == bf5rh_abi::UTC_NOT_SET {
                open_utc_page(st);
            } else {
                // Clock already set this session: go straight to the list.
                st.pending = Pending::Search;
            }
        }
    }
    AppResult::Continue
}

fn open_utc_page(st: &mut State) {
    st.page = Page::Utc;
    st.time.clear();
    if st.saved_date >= 0 {
        let (y, m, d) = sgp4mini::civil_from_days(st.saved_date);
        st.date.fill((y - 2000) as u32, m as u32, d as u32);
        // the date is already right nine times out of ten, so start on the
        // field that is always stale
        st.field = 1;
    } else {
        st.date.clear();
        st.field = 0;
    }
}

// ---- keys ----

fn key(api: &Api, kind: u8, id: u8) -> AppResult {
    if kind != KIND_SINGLE && kind != KIND_LONG && kind != KIND_REPEAT {
        return AppResult::Continue;
    }
    let st = state();
    st.status = ST_OK;
    match st.page {
        Page::Utc => key_utc(api, st, id),
        Page::Passes => key_passes(st, kind, id),
    }
}

fn key_passes(st: &mut State, kind: u8, id: u8) -> AppResult {
    let total = st.pass_count as usize;
    match id {
        KEY_EXIT if kind == KIND_SINGLE => return AppResult::Exit,
        KEY_ASTERISK if kind == KIND_SINGLE => open_utc_page(st),
        KEY_UP if total > 0 => st.pass_row = (st.pass_row + total - 1) % total,
        KEY_DOWN if total > 0 => st.pass_row = (st.pass_row + 1) % total,
        KEY_MENU if kind == KIND_SINGLE && total > 0 => {
            // Every track needs a fresh calibration table: the pass's samples
            // are for the clock as it stood when they were taken, and the only
            // cheap way to be sure they are fresh is to re-sample here.
            st.pending = Pending::Calibrate;
        }
        _ => {}
    }
    AppResult::Continue
}

fn key_utc(api: &Api, st: &mut State, id: u8) -> AppResult {
    match id {
        KEY_UP | KEY_DOWN => st.field ^= 1,
        KEY_MENU => return commit_utc(api, st),
        KEY_EXIT => {
            let input = if st.field == 0 {
                &mut st.date
            } else {
                &mut st.time
            };
            if input.is_empty() {
                // Only a `*`-reopen from an existing list has somewhere to go
                // back to; a fresh launch (no list predicted yet) exits the
                // app outright instead of landing on an empty Passes page
                // that would just need a second EXIT to leave.
                if st.pass_count > 0 {
                    st.page = Page::Passes;
                } else {
                    return AppResult::Exit;
                }
            } else {
                input.backspace();
            }
        }
        _ => {
            if let Some(digit) = digit_value(id) {
                let input = if st.field == 0 {
                    &mut st.date
                } else {
                    &mut st.time
                };
                if input.is_full() {
                    input.clear();
                }
                input.push(digit);
                // filling the date rolls on to the time, which is the field
                // that always has to be retyped
                if input.is_full() && st.field == 0 {
                    st.field = 1;
                }
            }
        }
    }
    AppResult::Continue
}

/// Rejects an impossible date or time rather than clamping it: a clamped
/// clock looks set and predicts confidently wrong passes.
fn commit_utc(api: &Api, st: &mut State) -> AppResult {
    if !st.date.is_full() || !st.time.is_full() {
        return AppResult::Continue;
    }
    let (yy, mo, dd) = st.date.pairs();
    let (hh, mi, ss) = st.time.pairs();
    if hh >= 24 || mi >= 60 || ss >= 60 || mo == 0 || mo > 12 || dd == 0 {
        return AppResult::Continue;
    }
    let (y, m, d) = (2000 + yy as i32, mo as i32, dd as i32);
    let days = sgp4mini::days_from_civil(y, m, d);
    // round trip rather than carry a month-length table: 02-31 comes back as
    // 03-02 and is refused
    if sgp4mini::civil_from_days(days) != (y, m, d) || days < 0 {
        return AppResult::Continue;
    }

    (api.utc_set)(days as u32 * SECS_PER_DAY + hh * 3600 + mi * 60 + ss);
    if days != st.saved_date {
        save_date(api, days);
        st.saved_date = days;
    }
    // A new clock means a fresh horizon: predict everything, right now.
    st.pending = Pending::Search;
    AppResult::Continue
}

fn chain_predict(api: &Api, st: &State, job: u8) -> AppResult {
    // `aos`/`los` only matter to `JOB_CALIBRATE`; search finds them itself.
    let e = selected_pass(st).unwrap_or(PassEntry::BLANK);
    Handoff {
        job,
        sat_index: e.sat_index,
        list_row: st.pass_row as u8,
        pass_index: e.pass_index,
        aos: e.aos,
        los: e.los,
        ..Handoff::BLANK
    }
    .store(api);
    AppResult::Chain(SEG_PREDICT)
}

// ---- drawing ----

static mut ROWS: [ListRow; VISIBLE_ROWS] = [ListRow {
    label: [0; 16],
    value: [0; 18],
    has_value: false,
    cursor: -1,
}; VISIBLE_ROWS];

fn blank_rows() -> &'static mut [ListRow; VISIBLE_ROWS] {
    let rows = unsafe { &mut *core::ptr::addr_of_mut!(ROWS) };
    for r in rows.iter_mut() {
        r.label = [0; 16];
        r.value = [0; 18];
        r.has_value = false;
        r.cursor = -1;
    }
    rows
}

fn emit_list(
    api: &Api,
    title: &[u8],
    window_start: usize,
    selected: usize,
    total: usize,
) {
    let rows_ptr = core::ptr::addr_of!(ROWS) as *const ListRow;
    (api.draw_list)(
        title.as_ptr(),
        title.len() as u16,
        rows_ptr,
        VISIBLE_ROWS as u16,
        window_start as u16,
        selected as u16,
        total as u16,
        false,
    );
}

fn text(api: &Api, x: i16, y: i16, s: &[u8], fg: u16, bg: u16, font: u8) {
    (api.draw_text)(x, y, s.as_ptr(), s.len() as u16, fg, bg, font);
}

const fn rgb(r: u16, g: u16, b: u16) -> u16 {
    (r << 11) | (g << 5) | b
}

fn draw_predict_banner(api: &Api, job: u8) {
    let msg: &[u8] = if job == JOB_SEARCH {
        b"PREDICTING..."
    } else {
        b"CALCULATING..."
    };
    let bg = rgb(2, 4, 9);
    let accent = rgb(0, 48, 24);
    let w = msg.len() as i16 * 6 + 20;
    let h = 28;
    let x = (SCREEN_W - w) / 2;
    let y = (SCREEN_H - h) / 2;
    (api.fill_rect)(x, y, w as u16, h as u16, bg);
    // A 2px accent frame reads as a card, not a wall of text.
    (api.fill_rect)(x, y, w as u16, 2, accent);
    (api.fill_rect)(x, y + h - 2, w as u16, 2, accent);
    (api.fill_rect)(x, y, 2, h as u16, accent);
    (api.fill_rect)(x + w - 2, y, 2, h as u16, accent);
    text(api, x + 10, y + (h - 10) / 2, msg, WHITE, bg, F6);
}

fn draw(api: &Api) -> AppResult {
    let st = state();

    // The predict segment cannot draw, so the banner has to be on screen
    // before the chain, which means chaining out of Draw rather than Key.
    if st.pending != Pending::None {
        let job = if st.pending == Pending::Search {
            JOB_SEARCH
        } else {
            JOB_CALIBRATE
        };
        st.pending = Pending::None;
        draw_predict_banner(api, job);
        return chain_predict(api, st, job);
    }

    match st.page {
        Page::Utc => draw_utc(api, st),
        Page::Passes => draw_passes(api, st),
    }
    draw_status(api, st);
    AppResult::Continue
}

fn draw_utc(api: &Api, st: &State) {
    let rows = blank_rows();
    let (yy, mo, dd) = st.date.pairs();
    let (hh, mi, ss) = st.time.pairs();

    write_str(&mut rows[0].label, "Date");
    if st.date.is_full() {
        let i = put_u32(&mut rows[0].value, 0, 2000 + yy, 4);
        let i = put_char(&mut rows[0].value, i, b'-');
        let i = put_u32(&mut rows[0].value, i, mo, 2);
        let i = put_char(&mut rows[0].value, i, b'-');
        put_u32(&mut rows[0].value, i, dd, 2);
    } else {
        put_pattern(&mut rows[0].value, &st.date, b"--/--/--");
    }
    rows[0].has_value = true;

    write_str(&mut rows[1].label, "Time UTC");
    if st.time.is_full() {
        put_hms(&mut rows[1].value, 0, hh * 3600 + mi * 60 + ss);
    } else {
        put_pattern(&mut rows[1].value, &st.time, b"--:--:--");
    }
    rows[1].has_value = true;

    emit_list(api, b"SET UTC", 0, st.field as usize, 2);
    text(api, 6, 112, b"MENU=OK  EXIT=DEL", WHITE, BLACK, F6);
}

/// Renders the typed digits into a `--/--/--` shaped template, leaving the
/// dashes wherever nothing has been typed yet.
fn put_pattern(out: &mut [u8], input: &DigitInput, pattern: &[u8]) {
    let mut d = 0usize;
    for (i, &c) in pattern.iter().enumerate() {
        if i >= out.len() {
            return;
        }
        out[i] = if c == b'-' {
            let ch = if d < input.len as usize {
                b'0' + input.digits[d]
            } else {
                b'-'
            };
            d += 1;
            ch
        } else {
            c
        };
    }
}

fn draw_passes(api: &Api, st: &State) {
    let mut title = [0u8; 16];
    match observer(api) {
        Some((lat, lon)) => {
            let i = put_coord(&mut title, 0, lat, true);
            let i = put_char(&mut title, i, b' ');
            put_coord(&mut title, i, lon, false);
        }
        None => write_str(&mut title, "PASSES"),
    }

    let total = st.pass_count as usize;
    let window_start = scroll_top(total, st.pass_row);
    let rows = blank_rows();
    let buf = unsafe { &*core::ptr::addr_of!(PASSES) };
    let ord = unsafe { &*core::ptr::addr_of!(ORDER) };
    for i in 0..total.min(VISIBLE_ROWS) {
        let gi = window_start + i;
        let e = &buf[ord[gi] as usize];
        // label: the satellite's name
        match read_head(api, e.sat_index as usize) {
            Some(h) => write_bytes(&mut rows[i].label, &h.name[..h.name_len()]),
            None => write_str(&mut rows[i].label, "?"),
        }
        // value: "MM-DD HH:MM ELxx"
        let j = put_md(&mut rows[i].value, 0, (e.aos / SECS_PER_DAY) as i32);
        let j = put_char(&mut rows[i].value, j, b' ');
        let j = put_hm(&mut rows[i].value, j, e.aos % SECS_PER_DAY);
        let j = put_char(&mut rows[i].value, j, b' ');
        let j = write_at(&mut rows[i].value, j, b"EL");
        put_i32(&mut rows[i].value, j, e.max_el_cdeg as i32 / 100, 0);
        rows[i].has_value = true;
    }
    emit_list(api, &title, window_start, st.pass_row, total);

    if st.occupancy == 0 {
        (api.fill_rect)(8, 50, 144, 30, WHITE);
        text(api, 14, 56, b"No satellites.", BLACK, WHITE, F6);
        text(api, 14, 68, b"Upload with push_sat", BLACK, WHITE, F5);
    } else if total > 0 && st.tle_age >= STALE_TLE_DAYS {
        let mut msg = [0u8; 20];
        let i = write_at(&mut msg, 0, b"TLE ");
        let i = put_u32(&mut msg, i, st.tle_age as u32, 0);
        let i = write_at(&mut msg, i, b"d old");
        text(api, 6, 112, &msg[..i], WHITE, BLACK, F6);
    }
}

/// Elements older than this predict noticeably late; ten days is where a low
/// orbit's along-track error starts to run into whole seconds.
const STALE_TLE_DAYS: u8 = 10;

fn draw_status(api: &Api, st: &State) {
    let msg: &[u8] = match st.status {
        ST_NO_UTC => b"Set the UTC clock",
        ST_NO_POS => b"Set LAT/LON in menu",
        ST_BAD_RECORD => b"Bad record, re-upload",
        ST_NO_PASS => b"No pass in 24h",
        ST_DECAYED => b"Orbit decayed",
        _ => return,
    };
    (api.fill_rect)(4, 100, 152, 16, WHITE);
    text(api, 8, 104, msg, BLACK, WHITE, F6);
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
