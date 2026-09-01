use core::mem::MaybeUninit;

use bf5rh_abi::{
    Api, AppEntryFn, AppEvent, AppResult, ListRow, PackageHeader, ARENA_MAX,
    ARENA_SIZE, BUILD_HASH, HANDOFF_SIZE, MAGIC,
};
use cortex_m::peripheral::{SCB, SYST};
use embedded_graphics::mono_font::ascii::{FONT_5X8, FONT_6X10, FONT_9X18};
use embedded_graphics::mono_font::{MonoFont, MonoTextStyleBuilder};
use embedded_graphics::pixelcolor::{raw::RawU16, Rgb565};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, PrimitiveStyle, Rectangle};
use embedded_graphics::text::Text;

use super::convert::{
    modulation_from_raw, power_from_raw, subaudio_from_code,
    subaudio_from_index, subaudio_index, subaudio_to_code,
};
use super::{settings, App, ChVfoMode, Mode};
use crate::device::keypad::KeyEvent;
use crate::device::radio::{self, RawTone, SubAudio};
use crate::drivers::st7735::{St7735, HEIGHT, WIDTH};
use crate::flash_map::addr;
use crate::ui::{draw_list as draw_list_widget, Cache, ListSource};

pub(crate) const SLOT_COUNT: u8 = addr::OVERLAY_SLOT_COUNT;

/// 8-byte alignment
/// a segment's `.bss` starts wherever the arena does, and an app doing `f64`
/// maths puts doubles there. A bare `[u8; N]` has alignment 1, so without this
/// the arena would drift a byte at a time as the firmware's own `.bss` changes.
#[repr(C, align(8))]
struct Arena([u8; ARENA_SIZE]);

#[link_section = ".uninit.OVERLAY_ARENA"]
static mut ARENA: MaybeUninit<Arena> = MaybeUninit::uninit();

unsafe extern "C" {
    /// start of the stack region; the arena must end exactly here
    static __sheap: u32;
}

/// Stack that must still be free below SP once an oversized segment has taken
/// its extension. The loader's own callees (the NOR read) live in it, and so
/// does the whole compute call the segment makes from `Enter`.
const OVERSIZE_HEADROOM: usize = 1024;

const CANARY: u32 = 0x5A43_414E;
const CANARY_WORDS: usize = 4;

/// Bytes an oversized segment took above [`ARENA_SIZE`]; 0 when the resident
/// segment fits the plain arena.
static mut ARENA_EXT: usize = 0;

/// `Fault` codes the loader raises for itself; kept far from the line numbers
/// an app passes to `app_fault`.
const FAULT_OVERSIZE_RESIDENT: u32 = 0xE000_0001;
const FAULT_OVERSIZE_CONTINUE: u32 = 0xE000_0002;
const FAULT_STACK_CANARY: u32 = 0xE000_0003;

static mut NEEDS_FULL_CLEAR: bool = false;

/// context available in [`call`]
struct Ctx {
    app: *mut App<'static>,
    syst: *mut SYST,

    /// null ptr in non Draw event
    lcd: *mut St7735<'static>,
}

static mut CTX: Option<Ctx> = None;
static mut LOADED_ENTRY: Option<AppEntryFn> = None;

fn ctx() -> &'static mut Ctx {
    unsafe {
        (*core::ptr::addr_of_mut!(CTX))
            .as_mut()
            .expect("overlay api called with no active dispatch")
    }
}

fn app_ref() -> &'static mut App<'static> {
    unsafe { &mut *ctx().app }
}

fn syst_ref() -> &'static mut SYST {
    unsafe { &mut *ctx().syst }
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

enum LoadError {
    BadMagic,
    BadHash,
    BadSegment,
    TooBig,
    BadCrc,

    /// oversized segment refused: SP was already too low, or the arena does
    /// not actually butt up against the stack
    NoStack,
}

fn load_err_code(e: &LoadError) -> u8 {
    match e {
        LoadError::BadMagic => 1,
        LoadError::BadHash => 2,
        LoadError::BadSegment => 3,
        LoadError::TooBig => 4,
        LoadError::BadCrc => 5,
        LoadError::NoStack => 6,
    }
}

/// Last segment-load failure, shown by the launcher so a silent refusal is not
/// mistaken for a hang. 0 = none.
static mut LAST_LOAD_ERR: u8 = 0;

pub(crate) fn take_load_err() -> u8 {
    let v = unsafe { LAST_LOAD_ERR };
    unsafe { LAST_LOAD_ERR = 0 };
    v
}

fn arena_ptr() -> *mut u8 {
    unsafe { (*core::ptr::addr_of_mut!(ARENA)).as_mut_ptr() as *mut u8 }
}

/// Compile-time proof that the arena can hold a `f64` at offset 0.
const _: () = assert!(core::mem::align_of::<Arena>() >= 8);

fn sp_now() -> usize {
    let sp: usize;
    unsafe {
        core::arch::asm!(
            "mov {}, sp",
            out(reg) sp,
            options(nomem, nostack, preserves_flags)
        );
    }
    sp
}

/// First word above an oversized segment's top, rounded up to 4.
fn canary_at(ext: usize) -> *mut u32 {
    let top = arena_ptr() as usize + ARENA_SIZE + ext;
    ((top + 3) & !3) as *mut u32
}

fn write_canary(ext: usize) {
    let p = canary_at(ext);
    for i in 0..CANARY_WORDS {
        unsafe { p.add(i).write_volatile(CANARY) };
    }
}

fn canary_intact(ext: usize) -> bool {
    let p = canary_at(ext);
    (0..CANARY_WORDS).all(|i| unsafe { p.add(i).read_volatile() } == CANARY)
}

fn peek_header(app: &mut App, slot: u8) -> PackageHeader {
    let mut hdr_buf = [0u8; PackageHeader::SIZE];
    app.storage_mut()
        .norflash
        .read_bytes(addr::overlay_slot_addr(slot), &mut hdr_buf);
    PackageHeader::from_bytes(&hdr_buf)
}

pub(crate) fn slot_valid(app: &mut App, slot: u8) -> bool {
    let hdr = peek_header(app, slot);
    hdr.magic == MAGIC
        && hdr.build_hash == BUILD_HASH
        && hdr.segment(0).is_some()
}

pub(crate) fn slot_name(app: &mut App, slot: u8, w: &mut dyn core::fmt::Write) {
    if !slot_valid(app, slot) {
        let _ = write!(w, "EMPTY");
        return;
    }
    let hdr = peek_header(app, slot);
    let _ = write!(w, "{}", hdr.name_str());
}

/// Load segment `seg` of the package in `slot` into the arena. Every segment
/// links against the same arena address, so this overwrites whatever segment
/// was resident.
fn load_segment(
    app: &mut App,
    slot: u8,
    seg: u8,
) -> Result<AppEntryFn, LoadError> {
    let base = addr::overlay_slot_addr(slot);
    let hdr = peek_header(app, slot);

    if hdr.magic != MAGIC {
        return Err(LoadError::BadMagic);
    }
    if hdr.build_hash != BUILD_HASH {
        return Err(LoadError::BadHash);
    }
    let seg = *hdr.segment(seg).ok_or(LoadError::BadSegment)?;

    let image_len = seg.image_len as usize;
    let bss_len = seg.bss_len as usize;
    let total = image_len.saturating_add(bss_len);
    if total > ARENA_MAX
        || seg.offset as u32 + seg.image_len > addr::OVERLAY_SLOT_SIZE
    {
        return Err(LoadError::TooBig);
    }

    let arena_ptr = arena_ptr();
    let ext = total.saturating_sub(ARENA_SIZE);

    // The extension is the bottom of the stack, so admit it only after proving
    // the stack is not there yet, and leave a canary to catch it if
    // it comes down anyway. Refusing is the whole point: an oversized segment
    // that is loaded without the room does not fail visibly, it randomly
    // corrupts whatever frame it lands on.
    unsafe { ARENA_EXT = 0 };
    if ext > 0 {
        if arena_ptr as usize + ARENA_SIZE != &raw const __sheap as usize {
            return Err(LoadError::NoStack);
        }
        let guard = canary_at(ext) as usize + CANARY_WORDS * 4;
        if sp_now() < guard + OVERSIZE_HEADROOM {
            return Err(LoadError::NoStack);
        }
        write_canary(ext);
    }

    let image =
        unsafe { core::slice::from_raw_parts_mut(arena_ptr, image_len) };
    app.storage_mut()
        .norflash
        .read_bytes(base + seg.offset, image);

    if crc32(image) != seg.crc32 {
        return Err(LoadError::BadCrc);
    }

    if bss_len > 0 {
        unsafe { core::ptr::write_bytes(arena_ptr.add(image_len), 0, bss_len) };
    }

    // The NOR read and the bss clear both ran with the stack right above the
    // extension, so check before trusting it.
    if ext > 0 {
        if !canary_intact(ext) {
            return Err(LoadError::NoStack);
        }
        unsafe { ARENA_EXT = ext };
    }

    cortex_m::asm::dsb();
    cortex_m::asm::isb();

    // Thumb function pointer LSB must set 1。
    let entry_addr = (arena_ptr as u32 + seg.entry_off) | 1;
    let entry: AppEntryFn = unsafe {
        core::mem::transmute::<*const (), AppEntryFn>(entry_addr as *const ())
    };
    Ok(entry)
}

fn call(
    app: &mut App,
    syst: &mut SYST,
    lcd: *mut St7735<'static>,
    ev: AppEvent,
) -> AppResult {
    let entry = match unsafe { LOADED_ENTRY } {
        Some(e) => e,
        None => return AppResult::Fault(0),
    };
    // An oversized segment aliases the stack, so the main loop must never run
    // while it is resident. `enter_call` enforces that it chains straight out
    // of `Enter`; this is the backstop if it somehow did not.
    if unsafe { ARENA_EXT } > 0 && !matches!(ev, AppEvent::Enter) {
        return AppResult::Fault(FAULT_OVERSIZE_RESIDENT);
    }
    unsafe {
        CTX = Some(Ctx {
            app: app as *mut App<'_> as *mut App<'static>,
            syst: syst as *mut SYST,
            lcd,
        });
    }
    let result = entry(&API, ev);
    unsafe {
        CTX = None;
    }
    result
}

/// `Enter` for a freshly loaded segment, plus the two checks that make a
/// stack-aliasing extension safe: the canary above the extension has to
/// survive, and an oversized segment has to be gone by the time this returns.
fn enter_call(app: &mut App, syst: &mut SYST) -> AppResult {
    let result = call(app, syst, core::ptr::null_mut(), AppEvent::Enter);
    let ext = unsafe { ARENA_EXT };
    if ext == 0 {
        return result;
    }
    if !canary_intact(ext) {
        return AppResult::Fault(FAULT_STACK_CANARY);
    }
    match result {
        AppResult::Continue => AppResult::Fault(FAULT_OVERSIZE_CONTINUE),
        other => other,
    }
}

/// A segment switch re-enters the loader from inside a dispatch, so bound the
/// hops: a segment that chains straight out of its own `Enter` would otherwise
/// spin forever.
const MAX_CHAIN_HOPS: u8 = 4;

/// An app that owned the radio (a satellite tracker parks it on a Doppler-
/// corrected satellite frequency) must not leave it there, and must not leave
/// it keyed, so put the master VFO back on air unconditionally.
fn leave_app(app: &mut App, syst: &mut SYST) {
    unsafe {
        LOADED_ENTRY = None;
        HANDOFF_LEN = 0;
        ARENA_EXT = 0;
    }
    app.mode = Mode::AppMenu;
    unsafe {
        APP_TX_ENABLED = false;
        APP_HOLDS_RX = false;
        APP_BACKLIGHT_HOLD = false;
    }
    if app.is_transmitting() {
        app.set_ptt(syst, false);
    }
    app.sync_watching_to_master(syst);
    app.reset_key_idle();
}

fn handle_result(app: &mut App, syst: &mut SYST, result: AppResult) {
    let mut result = result;
    let mut hops = 0u8;
    loop {
        let seg = match result {
            AppResult::Continue => return,
            AppResult::Exit | AppResult::Fault(_) => {
                leave_app(app, syst);
                return;
            }
            AppResult::Chain(seg) => seg,
        };

        hops += 1;
        let slot = match app.mode {
            Mode::External(slot) if hops <= MAX_CHAIN_HOPS => slot,
            _ => {
                leave_app(app, syst);
                return;
            }
        };

        if app_holds_rx() {
            app.radio_mut().stop_for_app(syst);
        }

        match load_segment(app, slot, seg) {
            Ok(entry) => unsafe {
                LOADED_ENTRY = Some(entry);
                NEEDS_FULL_CLEAR = true;
                LIST_CACHE = None;
                // Per segment, not per app: the tracking segment may key the
                // transmitter, the editor it chains back to may not.
                APP_TX_ENABLED = false;
                APP_HOLDS_RX = false;
                APP_BACKLIGHT_HOLD = false;
            },
            Err(e) => {
                unsafe { LAST_LOAD_ERR = load_err_code(&e) };
                leave_app(app, syst);
                return;
            }
        }
        // deliberately keeps HANDOFF: that is how segments talk to each other
        result = enter_call(app, syst);
    }
}

pub(crate) fn enter(app: &mut App, syst: &mut SYST, slot: u8) {
    match load_segment(app, slot, 0) {
        Ok(entry) => {
            unsafe {
                LOADED_ENTRY = Some(entry);
                NEEDS_FULL_CLEAR = true;

                LIST_CACHE = None;
                HANDOFF_LEN = 0;
                APP_TX_ENABLED = false;
                APP_HOLDS_RX = false;
                APP_BACKLIGHT_HOLD = false;
            }
            app.mode = Mode::External(slot);

            // stop standby
            app.radio_mut().stop_for_app(syst);
            let result = enter_call(app, syst);
            handle_result(app, syst, result);
        }
        Err(e) => {
            unsafe { LAST_LOAD_ERR = load_err_code(&e) };
        }
    }
}

pub(crate) fn dispatch_key(app: &mut App, syst: &mut SYST, ev: KeyEvent) {
    let event = AppEvent::Key {
        id: ev.key as u8,
        kind: ev.kind as u8,
    };
    let result = call(app, syst, core::ptr::null_mut(), event);
    handle_result(app, syst, result);
}

pub(crate) fn tick(app: &mut App, syst: &mut SYST, dt_100us: u32) {
    let result = call(
        app,
        syst,
        core::ptr::null_mut(),
        AppEvent::Tick { dt_100us },
    );
    handle_result(app, syst, result);
}

pub(crate) fn draw(lcd: &mut St7735<'_>, app: &mut App, syst: &mut SYST) {
    let needs_clear =
        unsafe { core::ptr::read(core::ptr::addr_of!(NEEDS_FULL_CLEAR)) };
    if needs_clear {
        unsafe { NEEDS_FULL_CLEAR = false };
        Rectangle::new(Point::zero(), Size::new(WIDTH as u32, HEIGHT as u32))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
            .draw(lcd)
            .ok();
    }

    let lcd_ptr = lcd as *mut St7735<'_> as *mut St7735<'static>;
    let result = call(app, syst, lcd_ptr, AppEvent::Draw);
    handle_result(app, syst, result);
}

// --------
// API
// --------

#[inline(never)]
extern "C" fn api_uptime_100us() -> u32 {
    crate::hal::uptime::now() as u32
}

#[inline(never)]
extern "C" fn api_delay_ms(ms: u32) {
    crate::hal::delay::ms(syst_ref(), ms);
}

#[inline(never)]
extern "C" fn api_fill_rect(x: i16, y: i16, w: u16, h: u16, rgb565: u16) {
    let lcd_ptr = ctx().lcd;
    if lcd_ptr.is_null() {
        return;
    }
    let lcd = unsafe { &mut *lcd_ptr };
    let color = Rgb565::from(RawU16::new(rgb565));
    Rectangle::new(
        Point::new(x as i32, y as i32),
        Size::new(w as u32, h as u32),
    )
    .into_styled(PrimitiveStyle::with_fill(color))
    .draw(lcd)
    .ok();
}

#[inline(never)]
extern "C" fn api_blit(x: i16, y: i16, w: u16, h: u16, px: *const u16) {
    let lcd_ptr = ctx().lcd;
    if lcd_ptr.is_null() || px.is_null() {
        return;
    }
    let lcd = unsafe { &mut *lcd_ptr };
    let n = w as usize * h as usize;
    let pixels = unsafe { core::slice::from_raw_parts(px, n) };
    let area = Rectangle::new(
        Point::new(x as i32, y as i32),
        Size::new(w as u32, h as u32),
    );
    lcd.fill_contiguous(
        &area,
        pixels.iter().map(|&raw| Rgb565::from(RawU16::new(raw))),
    )
    .ok();
}

#[inline(never)]
extern "C" fn api_draw_line(x0: i16, y0: i16, x1: i16, y1: i16, rgb565: u16) {
    let lcd_ptr = ctx().lcd;
    if lcd_ptr.is_null() {
        return;
    }
    let lcd = unsafe { &mut *lcd_ptr };
    let color = Rgb565::from(RawU16::new(rgb565));
    Line::new(
        Point::new(x0 as i32, y0 as i32),
        Point::new(x1 as i32, y1 as i32),
    )
    .into_styled(PrimitiveStyle::with_stroke(color, 1))
    .draw(lcd)
    .ok();
}

#[inline(never)]
extern "C" fn api_draw_circle(
    x: i16,
    y: i16,
    r: u16,
    rgb565: u16,
    filled: bool,
) {
    let lcd_ptr = ctx().lcd;
    if lcd_ptr.is_null() {
        return;
    }
    let lcd = unsafe { &mut *lcd_ptr };
    let color = Rgb565::from(RawU16::new(rgb565));
    let style = if filled {
        PrimitiveStyle::with_fill(color)
    } else {
        PrimitiveStyle::with_stroke(color, 1)
    };

    let d = 2 * r as u32;
    Circle::new(Point::new(x as i32 - r as i32, y as i32 - r as i32), d)
        .into_styled(style)
        .draw(lcd)
        .ok();
}

fn draw_text_with_font(
    lcd: &mut St7735<'_>,
    s: &str,
    x: i16,
    y: i16,
    font: &MonoFont<'_>,
    fg: Rgb565,
    bg: Rgb565,
) -> u16 {
    let style = MonoTextStyleBuilder::new()
        .font(font)
        .text_color(fg)
        .background_color(bg)
        .build();
    let baseline_y = y as i32 + font.baseline as i32;
    Text::new(s, Point::new(x as i32, baseline_y), style)
        .draw(lcd)
        .ok();
    (x as i32 + font.character_size.width as i32 * s.chars().count() as i32)
        as u16
}

#[inline(never)]
extern "C" fn api_draw_text(
    x: i16,
    y: i16,
    text: *const u8,
    len: u16,
    fg: u16,
    bg: u16,
    font: u8,
) -> u16 {
    let lcd_ptr = ctx().lcd;
    if lcd_ptr.is_null() || text.is_null() {
        return x as u16;
    }
    let lcd = unsafe { &mut *lcd_ptr };
    let bytes = unsafe { core::slice::from_raw_parts(text, len as usize) };
    let s = core::str::from_utf8(bytes).unwrap_or("");
    let fg = Rgb565::from(RawU16::new(fg));
    let bg = Rgb565::from(RawU16::new(bg));
    match font {
        2 => draw_text_with_font(lcd, s, x, y, &FONT_9X18, fg, bg),
        1 => draw_text_with_font(lcd, s, x, y, &FONT_6X10, fg, bg),
        _ => draw_text_with_font(lcd, s, x, y, &FONT_5X8, fg, bg),
    }
}

#[inline(never)]
extern "C" fn api_set_rx_freq(hz: u32) {
    app_ref().radio_mut().set_frequency(hz);
}

#[inline(never)]
extern "C" fn api_set_tx_freq(hz: u32) {
    app_ref().radio_mut().set_tx_frequency(hz);
}

#[inline(never)]
extern "C" fn api_set_ptt(on: bool) {
    let (app, syst) = (app_ref(), syst_ref());
    app.set_ptt(syst, on);
}

#[inline(never)]
extern "C" fn api_read_rssi() -> u16 {
    let (app, syst) = (app_ref(), syst_ref());
    app.radio_mut().rssi(syst)
}

#[inline(never)]
extern "C" fn api_set_modulation(mode: u8) {
    app_ref()
        .radio_mut()
        .set_modulation(modulation_from_raw(mode));
}

#[inline(never)]
extern "C" fn api_set_power(level: u8) {
    app_ref().radio_mut().set_power(power_from_raw(level));
}

#[inline(never)]
extern "C" fn api_set_subaudio_tx(code: u16) {
    app_ref()
        .radio_mut()
        .set_subaudio_tx(subaudio_from_code(code));
}

#[inline(never)]
extern "C" fn api_set_subaudio_rx(code: u16) {
    app_ref()
        .radio_mut()
        .set_subaudio_rx(subaudio_from_code(code));
}

#[inline(never)]
extern "C" fn api_tone_on(hz_div10: u16) {
    let (app, syst) = (app_ref(), syst_ref());
    app.radio_mut().rtone_on(syst, hz_div10);
}

#[inline(never)]
extern "C" fn api_tone_off() {
    let (app, syst) = (app_ref(), syst_ref());
    app.radio_mut().rtone_off(syst);
}

#[inline(never)]
extern "C" fn api_set_speaker(on: bool) {
    app_ref().radio_mut().set_speaker(on);
}

#[inline(never)]
extern "C" fn api_read_mic_level() -> u8 {
    let (app, syst) = (app_ref(), syst_ref());
    app.radio_mut().read_mic_level(syst)
}

static mut FM_PARKED: bool = false;

#[inline(never)]
extern "C" fn api_fm_tune_khz(freq_khz: u32) {
    let (app, syst) = (app_ref(), syst_ref());
    if unsafe { !FM_PARKED } {
        app.radio_mut().park_for_fm(syst);
        unsafe { FM_PARKED = true };
    }
    app.fm_radio_mut().tune_khz(syst, freq_khz);
}

#[inline(never)]
extern "C" fn api_fm_seek(up: bool) {
    let (app, syst) = (app_ref(), syst_ref());
    app.fm_radio_mut().seek(syst, up);
}

#[inline(never)]
extern "C" fn api_fm_status() -> u32 {
    let (app, syst) = (app_ref(), syst_ref());
    let (complete, seek_failed, is_station, rssi) =
        app.fm_radio_mut().status(syst);
    (complete as u32)
        | ((seek_failed as u32) << 1)
        | ((is_station as u32) << 2)
        | ((rssi as u32) << 8)
}

#[inline(never)]
extern "C" fn api_fm_tuned_freq_khz() -> u32 {
    let (app, syst) = (app_ref(), syst_ref());
    app.fm_radio_mut().tuned_frequency_khz(syst)
}

#[inline(never)]
extern "C" fn api_fm_power_off() {
    let (app, syst) = (app_ref(), syst_ref());
    app.fm_radio_mut().power_off(syst);
    app.sync_watching_to_master(syst);
    app.reset_key_idle();
    unsafe { FM_PARKED = false };
}

#[inline(never)]
extern "C" fn api_fm_channels_load(buf: *mut u8, len: u16) -> bool {
    const N: usize = crate::flash_map::FM_CHANNEL_COUNT * 2;
    if buf.is_null() || (len as usize) < N {
        return false;
    }
    let storage = app_ref().storage_mut();
    let channels = storage.load_fm_channels().unwrap_or(
        [crate::flash_map::FM_CHANNEL_EMPTY;
            crate::flash_map::FM_CHANNEL_COUNT],
    );
    let slice = unsafe { core::slice::from_raw_parts_mut(buf, N) };
    for (pair, v) in slice.chunks_exact_mut(2).zip(channels.iter()) {
        pair.copy_from_slice(&v.to_le_bytes());
    }
    true
}

#[inline(never)]
extern "C" fn api_fm_channels_save(buf: *const u8, len: u16) -> bool {
    const N: usize = crate::flash_map::FM_CHANNEL_COUNT * 2;
    if buf.is_null() || (len as usize) < N {
        return false;
    }
    let slice = unsafe { core::slice::from_raw_parts(buf, N) };
    let mut channels = [crate::flash_map::FM_CHANNEL_EMPTY;
        crate::flash_map::FM_CHANNEL_COUNT];
    for (slot, pair) in channels.iter_mut().zip(slice.chunks_exact(2)) {
        *slot = u16::from_le_bytes([pair[0], pair[1]]);
    }
    app_ref().storage_mut().save_fm_channels(&channels);
    true
}

#[inline(never)]
extern "C" fn api_nor_read(addr: u32, buf: *mut u8, len: u32) -> bool {
    if buf.is_null() {
        return false;
    }
    let slice = unsafe { core::slice::from_raw_parts_mut(buf, len as usize) };
    app_ref().storage_mut().norflash.read_bytes(addr, slice);
    true
}

#[inline(never)]
extern "C" fn api_nor_write(addr: u32, buf: *const u8, len: u32) -> bool {
    if buf.is_null() || addr < self::addr::OVERLAY_APP_ADDR {
        return false;
    }
    let slice = unsafe { core::slice::from_raw_parts(buf, len as usize) };
    app_ref().storage_mut().norflash.write_bytes(addr, slice);
    true
}

#[inline(never)]
extern "C" fn api_nor_erase_sector(addr: u32) -> bool {
    if addr < self::addr::OVERLAY_APP_ADDR {
        return false;
    }
    app_ref().storage_mut().norflash.erase_sector(addr);
    true
}

#[inline(never)]
extern "C" fn api_fmt_u32(v: u32, out: *mut u8) -> u16 {
    use core::fmt::Write;
    if out.is_null() {
        return 0;
    }
    let mut buf: crate::ui::TextBuf<10> = crate::ui::TextBuf::new();
    let _ = write!(buf, "{v}");
    let s = buf.as_str().as_bytes();
    unsafe { core::ptr::copy_nonoverlapping(s.as_ptr(), out, s.len()) };
    s.len() as u16
}

#[inline(never)]
extern "C" fn api_fmt_freq(hz: u32, out: *mut u8) -> u16 {
    use core::fmt::Write;
    if out.is_null() {
        return 0;
    }
    let mhz = hz / 1_000_000;
    let frac = (hz % 1_000_000) / 10;
    let mut buf: crate::ui::TextBuf<16> = crate::ui::TextBuf::new();
    let _ = write!(buf, "{mhz}.{frac:05}");
    let s = buf.as_str().as_bytes();
    unsafe { core::ptr::copy_nonoverlapping(s.as_ptr(), out, s.len()) };
    s.len() as u16
}

#[inline(never)]
extern "C" fn api_chan_read(num: u16, out: *mut u8, out_len: u16) -> bool {
    if out.is_null() || (out_len as u32) < addr::CHAN_SIZE {
        return false;
    }
    let storage = app_ref().storage_mut();
    if storage.is_channel_empty(num) {
        return false;
    }
    let bytes = storage.read_channel(num).to_bytes();
    unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), out, bytes.len()) };
    true
}

#[inline(never)]
extern "C" fn api_settings_get(id: u16) -> u32 {
    let s = &app_ref().settings;
    match id {
        0 => s.sql_level as u32,
        1 => s.tot_level as u32,
        2 => s.vox_switch as u32,
        3 => s.vox_level as u32,
        4 => s.beeps_switch as u32,
        5 => s.obs_lat as u32,
        6 => s.obs_lon as u32,
        _ => 0,
    }
}

extern "C" fn api_utc_get() -> u32 {
    app_ref().utc_secs()
}

extern "C" fn api_utc_set(secs: u32) {
    app_ref().set_utc_secs(secs);
}

static mut LIST_CACHE: Option<Cache> = None;

struct BufferListSource<'a> {
    rows: &'a [ListRow],
    window_start: usize,
    total: usize,
}

impl<'a> BufferListSource<'a> {
    fn window_row(&self, index: usize) -> Option<&'a ListRow> {
        if index < self.window_start {
            return None;
        }
        self.rows.get(index - self.window_start)
    }
}

impl<'a> ListSource for BufferListSource<'a> {
    fn row_count(&mut self) -> usize {
        self.total
    }

    fn label(&mut self, index: usize, w: &mut dyn core::fmt::Write) {
        if let Some(row) = self.window_row(index) {
            let end = row
                .label
                .iter()
                .position(|&b| b == 0)
                .unwrap_or(row.label.len());
            if let Ok(s) = core::str::from_utf8(&row.label[..end]) {
                let _ = w.write_str(s);
            }
        }
    }

    fn value(&mut self, index: usize, w: &mut dyn core::fmt::Write) -> bool {
        if let Some(row) = self.window_row(index) {
            if row.has_value {
                let end = row
                    .value
                    .iter()
                    .position(|&b| b == 0)
                    .unwrap_or(row.value.len());
                if let Ok(s) = core::str::from_utf8(&row.value[..end]) {
                    let _ = w.write_str(s);
                }
                return true;
            }
        }
        false
    }

    fn cursor(&mut self, index: usize) -> Option<usize> {
        match self.window_row(index) {
            Some(row) if row.cursor >= 0 => Some(row.cursor as usize),
            _ => None,
        }
    }
}

#[inline(never)]
extern "C" fn api_chan_write(num: u16, buf: *const u8, len: u16) -> bool {
    if buf.is_null()
        || num > super::MAX_CHANNEL_NUM
        || (len as u32) < addr::CHAN_SIZE
    {
        return false;
    }
    let bytes =
        unsafe { core::slice::from_raw_parts(buf, addr::CHAN_SIZE as usize) };
    let mut arr = [0u8; addr::CHAN_SIZE as usize];
    arr.copy_from_slice(bytes);
    let channel = crate::flash_map::Channel::from_bytes(&arr);
    let app = app_ref();
    app.storage_mut().write_channel_rmw(num, &channel);
    app.refresh_channel_display(num);
    true
}

#[inline(never)]
extern "C" fn api_get_master_freq() -> u32 {
    app_ref().master_freq_hz()
}

#[inline(never)]
extern "C" fn api_draw_list(
    title: *const u8,
    title_len: u16,
    rows: *const ListRow,
    row_count: u16,
    window_start: u16,
    selected: u16,
    total: u16,
    show_arrows: bool,
) {
    let lcd_ptr = ctx().lcd;
    if lcd_ptr.is_null() {
        return;
    }
    let lcd = unsafe { &mut *lcd_ptr };

    // trunk to first 0
    let title_str = if title.is_null() {
        ""
    } else {
        let bytes =
            unsafe { core::slice::from_raw_parts(title, title_len as usize) };
        let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
        core::str::from_utf8(&bytes[..end]).unwrap_or("")
    };
    let rows_slice = if rows.is_null() {
        &[][..]
    } else {
        unsafe { core::slice::from_raw_parts(rows, row_count as usize) }
    };
    let mut source = BufferListSource {
        rows: rows_slice,
        window_start: window_start as usize,
        total: total as usize,
    };
    let cache = unsafe {
        let ptr = core::ptr::addr_of_mut!(LIST_CACHE);
        if (*ptr).is_none() {
            *ptr = Some(Cache::new());
        }
        (*ptr).as_mut().expect("list cache")
    };
    draw_list_widget(
        lcd,
        title_str,
        &mut source,
        selected as usize,
        show_arrows,
        cache,
    );
}

#[inline(never)]
extern "C" fn api_subaudio_index_of_code(code: u16) -> i32 {
    subaudio_index(subaudio_from_code(code))
}

#[inline(never)]
extern "C" fn api_subaudio_code_of_index(v: i32) -> u16 {
    subaudio_to_code(subaudio_from_index(v))
}

#[inline(never)]
extern "C" fn api_subaudio_format(v: i32, out: *mut u8, cap: u16) -> u16 {
    use core::fmt::Write;
    if out.is_null() || cap == 0 {
        return 0;
    }
    let sub = subaudio_from_index(v);
    let mut buf: crate::ui::TextBuf<16> = crate::ui::TextBuf::new();
    match sub {
        crate::device::radio::SubAudio::None => {
            let _ = write!(buf, "OFF");
        }
        crate::device::radio::SubAudio::Ctcss(hz) => {
            let _ = write!(buf, "{}.{}Hz", hz / 10, hz % 10);
        }
        crate::device::radio::SubAudio::Dcs { code, inverted } => {
            let _ = write!(
                buf,
                "D{:03o}{}",
                code,
                if inverted { "I" } else { "N" }
            );
        }
    }
    let s = buf.as_str().as_bytes();
    let n = s.len().min(cap as usize);
    unsafe { core::ptr::copy_nonoverlapping(s.as_ptr(), out, n) };
    n as u16
}

// Firmware-owned, so it outlives the arena across a segment switch.
static mut APP_HANDOFF: [u8; HANDOFF_SIZE] = [0; HANDOFF_SIZE];
static mut HANDOFF_LEN: u16 = 0;

#[inline(never)]
extern "C" fn api_handoff_write(buf: *const u8, len: u16) -> bool {
    if buf.is_null() || len as usize > HANDOFF_SIZE {
        return false;
    }
    unsafe {
        core::ptr::copy_nonoverlapping(
            buf,
            core::ptr::addr_of_mut!(APP_HANDOFF) as *mut u8,
            len as usize,
        );
        HANDOFF_LEN = len;
    }
    true
}

#[inline(never)]
extern "C" fn api_handoff_read(buf: *mut u8, len: u16) -> u16 {
    if buf.is_null() {
        return 0;
    }
    let n = unsafe { HANDOFF_LEN }.min(len) as usize;
    unsafe {
        core::ptr::copy_nonoverlapping(
            core::ptr::addr_of!(APP_HANDOFF) as *const u8,
            buf,
            n,
        );
    }
    n as u16
}

#[inline(never)]
extern "C" fn api_set_bandwidth(wide: bool) {
    app_ref().radio_mut().set_wide_bandwidth(wide);
}

#[inline(never)]
extern "C" fn api_set_sql_level(level: u8) {
    let (app, syst) = (app_ref(), syst_ref());
    app.radio_mut().set_sql_level(syst, level);
}

#[inline(never)]
extern "C" fn api_set_monitor(on: bool) {
    app_ref().radio_mut().set_monitor(on);
}

#[inline(never)]
extern "C" fn api_enter_rx() {
    let (app, syst) = (app_ref(), syst_ref());
    unsafe { APP_HOLDS_RX = true };
    app.radio_mut().enter_rx(syst);
}

#[inline(never)]
extern "C" fn api_retune_rx(hz: u32, wide: bool) {
    let (app, syst) = (app_ref(), syst_ref());
    app.radio_mut().retune_rx(syst, hz, wide);
}

#[inline(never)]
extern "C" fn api_read_rf_gains(out: *mut u16) -> bool {
    if out.is_null() {
        return false;
    }
    let (app, syst) = (app_ref(), syst_ref());
    let (lnas, lna, pga, if_gain) = app.radio_mut().read_rf_gains(syst);
    unsafe {
        out.write(lnas as u16);
        out.add(1).write(lna as u16);
        out.add(2).write(pga as u16);
        out.add(3).write(if_gain);
    }
    true
}

#[inline(never)]
extern "C" fn api_adjust_rf_gain(menu: u8, up: bool) {
    if menu > 3 {
        return;
    }
    let (app, syst) = (app_ref(), syst_ref());
    // ABI is 0-based to match the order `read_rf_gains` writes them out; the
    // driver numbers the same four stages from 1.
    app.radio_mut().adjust_rf_gain(syst, menu + 1, up);
}

/// Opt-in TX permission for the loaded app; see `Api::set_tx_enabled`.
static mut APP_TX_ENABLED: bool = false;

pub(super) fn tx_enabled() -> bool {
    unsafe { core::ptr::read(core::ptr::addr_of!(APP_TX_ENABLED)) }
}

#[inline(never)]
extern "C" fn api_set_tx_enabled(on: bool) {
    unsafe { APP_TX_ENABLED = on };
}

static mut APP_HOLDS_RX: bool = false;
static mut APP_BACKLIGHT_HOLD: bool = false;

pub(crate) fn app_holds_rx() -> bool {
    unsafe { core::ptr::read(core::ptr::addr_of!(APP_HOLDS_RX)) }
}

pub(crate) fn backlight_hold() -> bool {
    unsafe { core::ptr::read(core::ptr::addr_of!(APP_BACKLIGHT_HOLD)) }
}

#[inline(never)]
extern "C" fn api_set_backlight_hold(on: bool) {
    unsafe { APP_BACKLIGHT_HOLD = on };
}

#[inline(never)]
extern "C" fn api_tx_state() -> u32 {
    let app = app_ref();
    (app.is_transmitting() as u32) | ((app.tx_prohibited() as u32) << 1)
}

#[inline(never)]
extern "C" fn api_app_fault(line: u32) -> ! {
    let _ = line;
    SCB::sys_reset();
}

#[inline(never)]
extern "C" fn api_get_master_wide() -> bool {
    let app = app_ref();
    app.side_wide_band(app.master_index())
}

#[inline(never)]
extern "C" fn api_squelch_open() -> bool {
    app_ref().rssi_open()
}

#[inline(never)]
extern "C" fn api_set_subaudio_scan_filter(on: bool) {
    let (app, syst) = (app_ref(), syst_ref());
    app.radio_mut().set_subaudio_scan_filter(syst, on);
}

#[inline(never)]
extern "C" fn api_detect_subaudio() -> i32 {
    let (app, syst) = (app_ref(), syst_ref());
    match app.radio_mut().detect_subaudio_raw(syst) {
        RawTone::None => -1,
        RawTone::Ctcss(raw) => {
            let tenths = radio::ctcss_raw_to_tenths_hz(raw);
            match radio::find_standard_ctcss(tenths, &settings::CTCSS_TABLE) {
                Some(hz) => subaudio_index(SubAudio::Ctcss(hz)),
                None => -2,
            }
        }
        RawTone::Dcs(raw) => {
            match radio::find_standard_dcs(raw, &settings::DCS_TABLE) {
                Some(code) => subaudio_index(SubAudio::Dcs {
                    code,
                    inverted: false,
                }),
                None => -2,
            }
        }
    }
}

#[inline(never)]
extern "C" fn api_save_master_subaudio(code: u16, also_tx: bool) {
    let app = app_ref();
    let sub = subaudio_from_code(code);
    let m = app.master_index();
    app.sides[m].cfg.subaudio_rx = sub;
    if also_tx {
        app.sides[m].cfg.subaudio_tx = sub;
    }
    if matches!(app.sides[m].vfo_chan, ChVfoMode::Vfo) {
        app.save_vfo();
    }
}

#[inline(never)]
extern "C" fn api_freq_scan_enable() {
    let (app, syst) = (app_ref(), syst_ref());
    app.radio_mut().freq_scan_enable(syst);
}

#[inline(never)]
extern "C" fn api_freq_scan_disable() {
    let (app, syst) = (app_ref(), syst_ref());
    app.radio_mut().freq_scan_disable(syst);
}

#[inline(never)]
extern "C" fn api_check_freq_scan() -> u32 {
    let (app, syst) = (app_ref(), syst_ref());
    app.radio_mut().check_freq_scan(syst).unwrap_or(u32::MAX)
}

#[inline(never)]
extern "C" fn api_correct_measured_freq_word(raw_word: u32) -> u32 {
    app_ref().radio.correct_measured_freq_word(raw_word)
}

#[inline(never)]
extern "C" fn api_tune_search_candidate(freq_hz: u32, uhf_path: bool) {
    let (app, syst) = (app_ref(), syst_ref());
    app.radio_mut().tune_search_candidate(syst, freq_hz, uhf_path);
}

#[inline(never)]
extern "C" fn api_save_master_vfo(freq_hz: u32, subaudio_code: u16) {
    let app = app_ref();
    let sub = subaudio_from_code(subaudio_code);
    let m = app.master_index();
    let s = &mut app.sides[m];
    s.vfo_chan = ChVfoMode::Vfo;
    s.rx_freq_hz = freq_hz;
    s.cfg.subaudio_rx = sub;
    s.cfg.subaudio_tx = sub;
    s.refresh_cfg_freqs();
    app.save_vfo();
}

static API: Api = Api {
    uptime_100us: api_uptime_100us,
    delay_ms: api_delay_ms,
    fill_rect: api_fill_rect,
    blit: api_blit,
    draw_text: api_draw_text,
    draw_line: api_draw_line,
    draw_circle: api_draw_circle,
    set_rx_freq: api_set_rx_freq,
    set_tx_freq: api_set_tx_freq,
    set_ptt: api_set_ptt,
    read_rssi: api_read_rssi,
    set_modulation: api_set_modulation,
    set_power: api_set_power,
    set_subaudio_tx: api_set_subaudio_tx,
    set_subaudio_rx: api_set_subaudio_rx,
    fm_tune_khz: api_fm_tune_khz,
    fm_seek: api_fm_seek,
    fm_status: api_fm_status,
    fm_tuned_freq_khz: api_fm_tuned_freq_khz,
    fm_power_off: api_fm_power_off,
    fm_channels_load: api_fm_channels_load,
    fm_channels_save: api_fm_channels_save,
    tone_on: api_tone_on,
    tone_off: api_tone_off,
    set_speaker: api_set_speaker,
    read_mic_level: api_read_mic_level,
    nor_read: api_nor_read,
    nor_write: api_nor_write,
    nor_erase_sector: api_nor_erase_sector,
    fmt_u32: api_fmt_u32,
    fmt_freq: api_fmt_freq,
    chan_read: api_chan_read,
    settings_get: api_settings_get,
    app_fault: api_app_fault,
    chan_write: api_chan_write,
    get_master_freq: api_get_master_freq,
    draw_list: api_draw_list,
    subaudio_index_of_code: api_subaudio_index_of_code,
    subaudio_code_of_index: api_subaudio_code_of_index,
    subaudio_format: api_subaudio_format,
    handoff_write: api_handoff_write,
    handoff_read: api_handoff_read,
    set_bandwidth: api_set_bandwidth,
    set_sql_level: api_set_sql_level,
    set_monitor: api_set_monitor,
    enter_rx: api_enter_rx,
    retune_rx: api_retune_rx,
    read_rf_gains: api_read_rf_gains,
    adjust_rf_gain: api_adjust_rf_gain,
    tx_state: api_tx_state,
    set_tx_enabled: api_set_tx_enabled,
    utc_get: api_utc_get,
    utc_set: api_utc_set,
    set_backlight_hold: api_set_backlight_hold,
    get_master_wide: api_get_master_wide,
    squelch_open: api_squelch_open,
    set_subaudio_scan_filter: api_set_subaudio_scan_filter,
    detect_subaudio: api_detect_subaudio,
    save_master_subaudio: api_save_master_subaudio,
    freq_scan_enable: api_freq_scan_enable,
    freq_scan_disable: api_freq_scan_disable,
    check_freq_scan: api_check_freq_scan,
    correct_measured_freq_word: api_correct_measured_freq_word,
    tune_search_candidate: api_tune_search_candidate,
    save_master_vfo: api_save_master_vfo,
};
