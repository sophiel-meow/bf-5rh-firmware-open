use super::{icons, TextBuf, SCREEN_H, SCREEN_W};
use crate::app::{self, ChannelDisplayMode};
use crate::device::radio::{Modulation, Power, SubAudio};
use core::fmt::Write as _;
use embedded_graphics::mono_font::{
    ascii::{FONT_5X8, FONT_6X10, FONT_9X18},
    MonoTextStyle,
};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::Text;

const BG: Rgb565 = Rgb565::BLACK;
const FG: Rgb565 = Rgb565::WHITE;
const ACCENT: Rgb565 = Rgb565::YELLOW;
const ALERT: Rgb565 = Rgb565::RED;

const STATUS_HEIGHT: i32 = 12;
const ROW1_H: i32 = 12;
const ROW2_H: i32 = 20;
const ROW3_H: i32 = 12;
const BAND_HEIGHT: i32 = ROW1_H + ROW2_H + ROW3_H;
const S_METER_HEIGHT: i32 = 12;
const RIGHT_MARGIN: i32 = 4;

#[derive(Clone, Copy, PartialEq)]
pub(super) struct Snapshot {
    is_transmitting: bool,
    power_save: bool,
    dual_standby: bool,
    vox: bool,
    key_locked: bool,
    master_index: usize,
    dtmf_dial_active: bool,
    dtmf_dial_len: usize,
    channel_input_len: usize,
    freq_input_len: usize,
    tx_prohibited: bool,
    no_channels_notice: bool,
    sides: [SideSnapshot; 2],
}

pub(super) struct Cache {
    structural: Option<Snapshot>,
    battery_bars: Option<u8>,
    rx_status: [Option<RxStatusState>; 2],
    tx_elapsed: Option<u32>,
    meter_row_shown: bool,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            structural: None,
            battery_bars: None,
            rx_status: [None, None],
            tx_elapsed: None,
            meter_row_shown: false,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum RxStatusText {
    None,
    Rx,
    Tx,
    LastSignal,
}

#[derive(Clone, Copy, PartialEq)]
struct RxStatusState {
    marker: Option<bool>,
    text: RxStatusText,
}

#[derive(Clone, Copy, PartialEq)]
struct SideSnapshot {
    freq_hz: u32,
    is_channel_mode: bool,
    channel_num: u16,
    modulation: Modulation,
    power: Power,
    subaudio_tx: SubAudio,
    subaudio_rx: SubAudio,
    offset_hz: u32,
    freq_dir: u8,
    wide_band: bool,
    reversed: bool,
}

impl Snapshot {
    fn overlay_active(&self) -> bool {
        self.tx_prohibited || self.no_channels_notice
    }

    fn capture(app: &app::App) -> Self {
        let is_transmitting = app.is_transmitting();
        let master_index = app.master_index();
        let side = |i: usize| -> SideSnapshot {
            let transmitting_here = is_transmitting && i == master_index;
            let freq_hz = if transmitting_here {
                app.side_tx_freq_hz(i)
            } else {
                app.side_freq_hz(i)
            };
            SideSnapshot {
                freq_hz,
                is_channel_mode: app.side_is_channel_mode(i),
                channel_num: app.side_channel_num(i),
                modulation: app.side_modulation(i),
                power: app.side_power(i),
                subaudio_tx: app.side_subaudio_tx(i),
                subaudio_rx: app.side_subaudio_rx(i),
                offset_hz: app.side_offset_hz(i),
                freq_dir: app.side_freq_dir(i),
                wide_band: app.side_wide_band(i),
                reversed: app.side_reversed(i),
            }
        };
        Snapshot {
            is_transmitting,
            power_save: app.power_save_active(),
            dual_standby: app.dual_standby_enabled(),
            vox: app.vox_enabled(),
            key_locked: app.is_key_locked(),
            master_index,
            dtmf_dial_active: app.dtmf_dial_active(),
            dtmf_dial_len: app.dtmf_dial_len(),
            channel_input_len: app.channel_input_len(),
            freq_input_len: app.freq_input_len(),
            tx_prohibited: app.tx_prohibited(),
            no_channels_notice: app.no_channels_notice(),
            sides: [side(0), side(1)],
        }
    }
}

fn full_redraw<D>(lcd: &mut D, app: &app::App, top0: i32, top1: i32)
where
    D: DrawTarget<Color = Rgb565>,
{
    Rectangle::new(
        Point::new(0, 0),
        Size::new(SCREEN_W as u32, SCREEN_H as u32),
    )
    .into_styled(PrimitiveStyle::with_fill(BG))
    .draw(lcd)
    .ok();

    draw_status_bar(lcd, app);
    draw_vfo_band(lcd, app, 0, top0);
    draw_vfo_band(lcd, app, 1, top1);

    if app.tx_prohibited() {
        draw_overlay(lcd, "TX PROHIBITED", ALERT);
    } else if app.no_channels_notice() {
        draw_overlay(lcd, "NO CHANNELS", ACCENT);
    }
}

pub fn draw_standby<D>(lcd: &mut D, app: &app::App, cache: &mut Cache)
where
    D: DrawTarget<Color = Rgb565>,
{
    let snap = Snapshot::capture(app);
    let top0 = STATUS_HEIGHT;
    let top1 = STATUS_HEIGHT + BAND_HEIGHT + S_METER_HEIGHT;

    let overlay_active = snap.overlay_active();
    let prev_overlay_active = cache
        .structural
        .as_ref()
        .map(|p| p.overlay_active())
        .unwrap_or(false);
    let overlay_content_changed = cache.structural.as_ref().map_or(true, |p| {
        p.tx_prohibited != snap.tx_prohibited
            || p.no_channels_notice != snap.no_channels_notice
    });
    let first_frame = cache.structural.is_none();

    let did_full_redraw = if overlay_active {
        first_frame || !prev_overlay_active || overlay_content_changed
    } else {
        first_frame || prev_overlay_active
    };

    if did_full_redraw {
        full_redraw(lcd, app, top0, top1);
    } else if !overlay_active {
        let prev = cache.structural.as_ref().unwrap(); // Some, since !first_frame here

        let tx_state_changed = prev.is_transmitting != snap.is_transmitting;
        let master_changed = prev.master_index != snap.master_index;

        if snap.is_transmitting {
            draw_tx_timer(lcd, app, &mut cache.tx_elapsed, tx_state_changed);
        } else {
            if tx_state_changed {
                cache.tx_elapsed = None;
                clear_rect_x(lcd, TX_TIMER_X, 0, TX_TIMER_W, STATUS_HEIGHT);
            }
            if prev.power_save != snap.power_save || tx_state_changed {
                draw_ps_icon(lcd, app);
            }
            if prev.dual_standby != snap.dual_standby || tx_state_changed {
                draw_dwr_icon(lcd, app);
            }
        }
        if prev.vox != snap.vox {
            draw_vox_icon(lcd, app);
        }
        if prev.key_locked != snap.key_locked {
            draw_lock_icon(lcd, app);
        }

        for &(i, top) in &[(0usize, top0), (1usize, top1)] {
            let p = &prev.sides[i];
            let s = &snap.sides[i];
            let is_master = i == snap.master_index;
            let p_tx_here = prev.is_transmitting && i == prev.master_index;
            let s_tx_here = snap.is_transmitting && i == snap.master_index;

            let row1_input_changed = if is_master {
                prev.channel_input_len != snap.channel_input_len
            } else {
                prev.dtmf_dial_active != snap.dtmf_dial_active
            };
            if master_changed
                || row1_input_changed
                || (p.is_channel_mode, p.channel_num)
                    != (s.is_channel_mode, s.channel_num)
            {
                draw_row1(lcd, app, i, top);
            }

            let row2_input_changed = if is_master {
                prev.freq_input_len != snap.freq_input_len
            } else {
                prev.dtmf_dial_active != snap.dtmf_dial_active
                    || (snap.dtmf_dial_active
                        && prev.dtmf_dial_len != snap.dtmf_dial_len)
            };
            if master_changed
                || row2_input_changed
                || p.freq_hz != s.freq_hz
                || (p.is_channel_mode, p.channel_num)
                    != (s.is_channel_mode, s.channel_num)
            {
                draw_row2(lcd, app, i, top);
            }

            let row3_input_changed =
                !is_master && prev.dtmf_dial_active != snap.dtmf_dial_active;
            if master_changed
                || row3_input_changed
                || p_tx_here != s_tx_here
                || (
                    p.modulation,
                    p.power,
                    p.subaudio_tx,
                    p.subaudio_rx,
                    p.offset_hz,
                    p.freq_dir,
                    p.wide_band,
                    p.reversed,
                ) != (
                    s.modulation,
                    s.power,
                    s.subaudio_tx,
                    s.subaudio_rx,
                    s.offset_hz,
                    s.freq_dir,
                    s.wide_band,
                    s.reversed,
                )
            {
                draw_row3(lcd, app, i, top);
            }
        }
    }
    // else: overlay_active and unchanged from last tick -- already fully
    // painted, nothing structural to touch.

    cache.structural = Some(snap);

    draw_battery_icon(lcd, app, &mut cache.battery_bars, did_full_redraw);
    draw_rx_status(lcd, app, 0, top0, &mut cache.rx_status[0], did_full_redraw);
    draw_rx_status(lcd, app, 1, top1, &mut cache.rx_status[1], did_full_redraw);

    let show_mic = app.is_transmitting();
    let show_s_meter = !show_mic && app.audio_open();
    if show_mic {
        draw_mic_bar(lcd, app, top0 + BAND_HEIGHT);
    } else if show_s_meter {
        draw_s_meter(lcd, app, top0 + BAND_HEIGHT);
    } else if cache.meter_row_shown {
        clear_row(lcd, top0 + BAND_HEIGHT);
    }
    cache.meter_row_shown = show_mic || show_s_meter;
}

fn clear_rect<D>(lcd: &mut D, y: i32, h: i32)
where
    D: DrawTarget<Color = Rgb565>,
{
    Rectangle::new(Point::new(0, y), Size::new(SCREEN_W as u32, h as u32))
        .into_styled(PrimitiveStyle::with_fill(BG))
        .draw(lcd)
        .ok();
}

fn clear_rect_x<D>(lcd: &mut D, x: i32, y: i32, w: i32, h: i32)
where
    D: DrawTarget<Color = Rgb565>,
{
    Rectangle::new(Point::new(x, y), Size::new(w as u32, h as u32))
        .into_styled(PrimitiveStyle::with_fill(BG))
        .draw(lcd)
        .ok();
}

fn draw_overlay<D>(lcd: &mut D, text: &str, bg: Rgb565)
where
    D: DrawTarget<Color = Rgb565>,
{
    let width = text.chars().count() as i32
        * FONT_6X10.character_size.width as i32
        + 12;
    let x = (SCREEN_W - width) / 2;
    let y = SCREEN_H / 2 - 10;
    Rectangle::new(Point::new(x, y), Size::new(width as u32, 20))
        .into_styled(PrimitiveStyle::with_fill(bg))
        .draw(lcd)
        .ok();
    Text::new(
        text,
        Point::new(x + 6, y + FONT_6X10.baseline as i32 + 5),
        MonoTextStyle::new(&FONT_6X10, BG),
    )
    .draw(lcd)
    .ok();
}

fn draw_status_bar<D>(lcd: &mut D, app: &app::App)
where
    D: DrawTarget<Color = Rgb565>,
{
    if app.is_transmitting() {
        render_tx_timer(lcd, app);
    } else {
        draw_ps_icon(lcd, app);
        draw_dwr_icon(lcd, app);
    }
    draw_vox_icon(lcd, app);
    draw_lock_icon(lcd, app);
}

const PS_X: i32 = 2;
const PS_W: i32 = 16;
const DWR_X: i32 = 20;
const DWR_W: i32 = 20;
const VOX_X: i32 = 70;
const VOX_W: i32 = 20;
const LOCK_X: i32 = 100;
const LOCK_W: i32 = 26;

const TX_TIMER_X: i32 = PS_X;
const TX_TIMER_W: i32 = DWR_X + DWR_W - PS_X;

fn render_tx_timer<D>(lcd: &mut D, app: &app::App)
where
    D: DrawTarget<Color = Rgb565>,
{
    clear_rect_x(lcd, TX_TIMER_X, 0, TX_TIMER_W, STATUS_HEIGHT);
    let by = FONT_5X8.baseline as i32 + 1;
    let secs = app.tx_elapsed_seconds();
    let mut buf: TextBuf<8> = TextBuf::new();
    write!(buf, "{:02}:{:02}", secs / 60, secs % 60).ok();
    Text::new(
        buf.as_str(),
        Point::new(TX_TIMER_X, by),
        MonoTextStyle::new(&FONT_5X8, ACCENT),
    )
    .draw(lcd)
    .ok();
}

fn draw_tx_timer<D>(
    lcd: &mut D,
    app: &app::App,
    last: &mut Option<u32>,
    force: bool,
) where
    D: DrawTarget<Color = Rgb565>,
{
    let secs = app.tx_elapsed_seconds();
    if !force && *last == Some(secs) {
        return;
    }
    *last = Some(secs);
    render_tx_timer(lcd, app);
}

fn draw_ps_icon<D>(lcd: &mut D, app: &app::App)
where
    D: DrawTarget<Color = Rgb565>,
{
    clear_rect_x(lcd, PS_X, 0, PS_W, STATUS_HEIGHT);
    if app.power_save_active() {
        let by = FONT_5X8.baseline as i32 + 1;
        Text::new(
            "PS",
            Point::new(PS_X, by),
            MonoTextStyle::new(&FONT_5X8, FG),
        )
        .draw(lcd)
        .ok();
    }
}

fn draw_dwr_icon<D>(lcd: &mut D, app: &app::App)
where
    D: DrawTarget<Color = Rgb565>,
{
    clear_rect_x(lcd, DWR_X, 0, DWR_W, STATUS_HEIGHT);
    if app.dual_standby_enabled() {
        let by = FONT_5X8.baseline as i32 + 1;
        Text::new(
            "DWR",
            Point::new(DWR_X, by),
            MonoTextStyle::new(&FONT_5X8, FG),
        )
        .draw(lcd)
        .ok();
    }
}

fn draw_vox_icon<D>(lcd: &mut D, app: &app::App)
where
    D: DrawTarget<Color = Rgb565>,
{
    clear_rect_x(lcd, VOX_X, 0, VOX_W, STATUS_HEIGHT);
    if app.vox_enabled() {
        let by = FONT_5X8.baseline as i32 + 1;
        Text::new(
            "VOX",
            Point::new(VOX_X, by),
            MonoTextStyle::new(&FONT_5X8, FG),
        )
        .draw(lcd)
        .ok();
    }
}

fn draw_lock_icon<D>(lcd: &mut D, app: &app::App)
where
    D: DrawTarget<Color = Rgb565>,
{
    clear_rect_x(lcd, LOCK_X, 0, LOCK_W, STATUS_HEIGHT);
    if app.is_key_locked() {
        icons::draw_key_lock(lcd, LOCK_X, 0, FG);
    }
}

const BATTERY_ICON_W: i32 = 22;
const BATTERY_ICON_H: i32 = 10;

fn draw_battery_icon<D>(
    lcd: &mut D,
    app: &app::App,
    last: &mut Option<u8>,
    force: bool,
) where
    D: DrawTarget<Color = Rgb565>,
{
    let bars = app.battery_bars();
    if !force && *last == Some(bars) {
        return;
    }
    *last = Some(bars);

    let x = SCREEN_W - RIGHT_MARGIN - BATTERY_ICON_W;
    Rectangle::new(
        Point::new(x, 0),
        Size::new(BATTERY_ICON_W as u32, BATTERY_ICON_H as u32),
    )
    .into_styled(PrimitiveStyle::with_fill(BG))
    .draw(lcd)
    .ok();
    icons::draw_battery(lcd, SCREEN_W - RIGHT_MARGIN, 0, bars + 2, FG);
}

const ROW1_STATUS_W: i32 = 40; // must stay clear of the chan label at x=40, see `draw_vfo_band`

fn draw_rx_status<D>(
    lcd: &mut D,
    app: &app::App,
    i: usize,
    top: i32,
    last: &mut Option<RxStatusState>,
    force: bool,
) where
    D: DrawTarget<Color = Rgb565>,
{
    let is_master = i == app.master_index();
    let is_watching = i == app.watching_index();
    let transmitting_here = app.is_transmitting() && is_master;

    let marker = if is_master {
        Some(true)
    } else if is_watching && app.audio_open() {
        Some(false)
    } else {
        None
    };
    let text = if transmitting_here {
        RxStatusText::Tx
    } else if is_watching && (app.is_monitor() || app.rssi_open()) {
        if app.rx_blink_on() {
            RxStatusText::Rx
        } else {
            RxStatusText::None
        }
    } else if app.last_signal_side() == Some(i) {
        RxStatusText::LastSignal
    } else {
        RxStatusText::None
    };

    let state = RxStatusState { marker, text };
    if !force && *last == Some(state) {
        return;
    }
    *last = Some(state);

    Rectangle::new(
        Point::new(0, top),
        Size::new(ROW1_STATUS_W as u32, ROW1_H as u32),
    )
    .into_styled(PrimitiveStyle::with_fill(BG))
    .draw(lcd)
    .ok();

    if let Some(filled) = marker {
        icons::draw_vfo_marker(lcd, 8, top + ROW1_H / 2, filled, FG);
    }

    let row1_baseline = top + FONT_6X10.baseline as i32 + 1;
    let (label, color) = match text {
        RxStatusText::Tx => ("TX", ALERT),
        RxStatusText::Rx => ("RX", ACCENT),
        RxStatusText::LastSignal => (">>", ACCENT),
        RxStatusText::None => ("", ACCENT),
    };
    Text::new(
        label,
        Point::new(16, row1_baseline),
        MonoTextStyle::new(&FONT_6X10, color),
    )
    .draw(lcd)
    .ok();
}

fn draw_vfo_band<D>(lcd: &mut D, app: &app::App, i: usize, top: i32)
where
    D: DrawTarget<Color = Rgb565>,
{
    draw_row1(lcd, app, i, top);
    draw_row2(lcd, app, i, top);
    draw_row3(lcd, app, i, top);
}

fn draw_row1<D>(lcd: &mut D, app: &app::App, i: usize, top: i32)
where
    D: DrawTarget<Color = Rgb565>,
{
    clear_rect_x(lcd, ROW1_STATUS_W, top, SCREEN_W - ROW1_STATUS_W, ROW1_H);
    let is_master = i == app.master_index();
    let row1_baseline = top + FONT_6X10.baseline as i32 + 1;

    if is_master || !app.dtmf_dial_active() {
        if is_master && app.channel_input_len() > 0 {
            draw_channel_number_input(lcd, app, row1_baseline);
        } else {
            let mut chan_label: TextBuf<8> = TextBuf::new();
            if app.side_is_channel_mode(i) {
                write!(chan_label, "M{}", app.side_channel_num(i)).ok();
            } else {
                write!(chan_label, "VFO").ok();
            }
            Text::new(
                chan_label.as_str(),
                Point::new(40, row1_baseline),
                MonoTextStyle::new(&FONT_6X10, FG),
            )
            .draw(lcd)
            .ok();
        }
    }
}

fn draw_row2<D>(lcd: &mut D, app: &app::App, i: usize, top: i32)
where
    D: DrawTarget<Color = Rgb565>,
{
    clear_rect(lcd, top + ROW1_H, ROW2_H);
    let is_master = i == app.master_index();
    let transmitting_here = app.is_transmitting() && is_master;
    let row2_baseline = top + ROW1_H + FONT_9X18.baseline as i32;
    let channel_mode = app.side_is_channel_mode(i);
    let name = app.side_name_str(i);
    let has_name = channel_mode && !name.is_empty();

    let freq_hz = if transmitting_here {
        app.side_tx_freq_hz(i)
    } else {
        app.side_freq_hz(i)
    };

    if is_master && app.freq_input_len() > 0 {
        draw_freq_input(lcd, app, row2_baseline);
    } else if !is_master && app.dtmf_dial_active() {
        draw_dtmf_dial_input(
            lcd,
            app,
            top + ROW1_H + FONT_6X10.baseline as i32,
        );
    } else {
        match app.channel_display_mode() {
            ChannelDisplayMode::NameFreq if has_name => {
                let shown = &name[..name.len().min(16)];
                let name_y = top + ROW1_H + FONT_6X10.baseline as i32;
                draw_right_aligned(lcd, shown, &FONT_6X10, name_y, FG);
                let mhz = freq_hz / 1_000_000;
                let frac = (freq_hz % 1_000_000) / 10;
                let mut freq_line: TextBuf<12> = TextBuf::new();
                write!(freq_line, "{:3}.{:05}", mhz, frac).ok();
                draw_right_aligned(
                    lcd,
                    freq_line.as_str(),
                    &FONT_6X10,
                    name_y + FONT_6X10.character_size.height as i32,
                    FG,
                );
            }
            ChannelDisplayMode::Name if has_name => {
                let shown = &name[..name.len().min(16)];
                draw_right_aligned(lcd, shown, &FONT_9X18, row2_baseline, FG);
            }
            _ => {
                draw_frequency(lcd, freq_hz, row2_baseline);
            }
        }
    }
}

fn draw_row3<D>(lcd: &mut D, app: &app::App, i: usize, top: i32)
where
    D: DrawTarget<Color = Rgb565>,
{
    clear_rect(lcd, top + ROW1_H + ROW2_H, ROW3_H);
    let is_master = i == app.master_index();
    let transmitting_here = app.is_transmitting() && is_master;
    let row3_baseline = top + ROW1_H + ROW2_H + FONT_6X10.baseline as i32;
    if is_master || !app.dtmf_dial_active() {
        if transmitting_here {
            draw_tx_row(lcd, app, top + ROW1_H + ROW2_H);
        } else {
            draw_mode_line(lcd, app, i, row3_baseline);
        }
    }
}

pub(super) fn draw_right_aligned<D>(
    lcd: &mut D,
    text: &str,
    font: &embedded_graphics::mono_font::MonoFont,
    y: i32,
    color: Rgb565,
) where
    D: DrawTarget<Color = Rgb565>,
{
    let width = text.chars().count() as i32 * font.character_size.width as i32;
    let x = SCREEN_W - RIGHT_MARGIN - width;
    Text::new(text, Point::new(x, y), MonoTextStyle::new(font, color))
        .draw(lcd)
        .ok();
}

pub(crate) fn draw_frequency<D>(lcd: &mut D, freq_hz: u32, baseline_y: i32)
where
    D: DrawTarget<Color = Rgb565>,
{
    let mhz = freq_hz / 1_000_000;
    let frac = (freq_hz % 1_000_000) / 10; // 5 fractional digits (kHz + 2 extra)
    let khz = frac / 100;
    let tail = frac % 100;

    let mut big: TextBuf<8> = TextBuf::new();
    write!(big, "{:3}.{:03}", mhz, khz).ok();
    let mut small: TextBuf<4> = TextBuf::new();
    write!(small, "{:02}", tail).ok();

    let big_width =
        big.as_str().len() as i32 * FONT_9X18.character_size.width as i32;
    let small_width =
        small.as_str().len() as i32 * FONT_6X10.character_size.width as i32;
    let big_x = SCREEN_W - RIGHT_MARGIN - small_width - big_width;

    Text::new(
        big.as_str(),
        Point::new(big_x, baseline_y),
        MonoTextStyle::new(&FONT_9X18, FG),
    )
    .draw(lcd)
    .ok();
    Text::new(
        small.as_str(),
        Point::new(
            big_x + big_width,
            baseline_y - FONT_9X18.baseline as i32 + FONT_6X10.baseline as i32,
        ),
        MonoTextStyle::new(&FONT_6X10, FG),
    )
    .draw(lcd)
    .ok();
}

/// Mid-entry VFO frequency: `"xxx.x--"`
fn draw_freq_input<D>(lcd: &mut D, app: &app::App, baseline_y: i32)
where
    D: DrawTarget<Color = Rgb565>,
{
    let len = app.freq_input_len();
    let mut buf: TextBuf<8> = TextBuf::new();
    for pos in 0..6usize {
        if pos == 3 {
            write!(buf, ".").ok();
        }
        if pos < len {
            write!(buf, "{}", app.freq_input_digit(pos) as u32).ok();
        } else {
            write!(buf, "-").ok();
        }
    }
    draw_right_aligned(lcd, buf.as_str(), &FONT_9X18, baseline_y, ACCENT);
}

fn draw_channel_number_input<D>(lcd: &mut D, app: &app::App, y: i32)
where
    D: DrawTarget<Color = Rgb565>,
{
    let len = app.channel_input_len();
    let mut buf: TextBuf<8> = TextBuf::new();
    write!(buf, "M").ok();
    for pos in 0..3usize {
        if pos + len < 3 {
            write!(buf, "-").ok();
        } else {
            write!(buf, "{}", app.freq_input_digit(pos + len - 3) as u32).ok();
        }
    }
    Text::new(
        buf.as_str(),
        Point::new(40, y),
        MonoTextStyle::new(&FONT_6X10, ACCENT),
    )
    .draw(lcd)
    .ok();
}

fn draw_dtmf_dial_input<D>(lcd: &mut D, app: &app::App, baseline_y: i32)
where
    D: DrawTarget<Color = Rgb565>,
{
    let len = app.dtmf_dial_len();
    let cap = app.dtmf_dial_capacity();
    let mut buf: TextBuf<17> = TextBuf::new();
    write!(buf, ">").ok();
    for pos in 0..cap {
        if pos < len {
            write!(buf, "{}", dtmf_char(app.dtmf_dial_digit(pos))).ok();
        } else {
            write!(buf, "-").ok();
        }
    }
    Text::new(
        buf.as_str(),
        Point::new(4, baseline_y),
        MonoTextStyle::new(&FONT_6X10, ACCENT),
    )
    .draw(lcd)
    .ok();
}

fn tone_mode_label(tx: SubAudio, rx: SubAudio) -> &'static str {
    match (tx, rx) {
        (SubAudio::None, SubAudio::None) => "",
        (SubAudio::Ctcss(_), SubAudio::None) => "Tone",
        (SubAudio::Ctcss(t), SubAudio::Ctcss(r)) if t == r => "TSQL",
        (
            SubAudio::Dcs {
                code: tc,
                inverted: ti,
            },
            SubAudio::Dcs {
                code: rc,
                inverted: ri,
            },
        ) if tc == rc && ti == ri => "DTCS",
        (SubAudio::None, SubAudio::Dcs { .. }) => "DTCS-R",
        (SubAudio::None, SubAudio::Ctcss(_)) => "TSQL-R",
        _ => "Cross",
    }
}

fn draw_mode_line<D>(lcd: &mut D, app: &app::App, i: usize, baseline_y: i32)
where
    D: DrawTarget<Color = Rgb565>,
{
    let modulation = match app.side_modulation(i) {
        Modulation::Fm => "FM",
        Modulation::Am => "AM",
        Modulation::Usb => "USB",
        // Unreachable app-side (CW out of scope for this v1); kept only
        // because `Modulation` is shared with `drivers::fd6818b`.
        Modulation::Cw => "CW",
        Modulation::Cwf => "CWF",
    };
    let power = match app.side_power(i) {
        Power::Low => "LOW",
        Power::Mid => "MID",
        Power::High => "HIGH",
    };
    let tone =
        tone_mode_label(app.side_subaudio_tx(i), app.side_subaudio_rx(i));
    let dir = if app.side_offset_hz(i) == 0 {
        ""
    } else {
        match app.side_freq_dir(i) {
            1 => "+",
            2 => "-",
            _ => "",
        }
    };
    let bandwidth = if app.side_wide_band(i) { "WIDE" } else { "NAR" };
    let reversed = if app.side_reversed(i) { "R" } else { "" };

    let mut line: TextBuf<40> = TextBuf::new();
    for (idx, field) in [modulation, power, tone, dir, bandwidth, reversed]
        .into_iter()
        .filter(|f| !f.is_empty())
        .enumerate()
    {
        if idx > 0 {
            line.write_str(" ").ok();
        }
        line.write_str(field).ok();
    }
    Text::new(
        line.as_str(),
        Point::new(4, baseline_y),
        MonoTextStyle::new(&FONT_5X8, FG),
    )
    .draw(lcd)
    .ok();
}

fn dtmf_char(v: u8) -> char {
    match v {
        0..=9 => (b'0' + v) as char,
        10..=13 => (b'A' + (v - 10)) as char,
        14 => '*',
        15 => '#',
        _ => '?',
    }
}

const BAR_SEG_PITCH: i32 = 6;
const BAR_SEG_W: i32 = 5;
const BAR_MAX_H: i32 = 8;
const BAR_HOLLOW_COUNT: u8 = 4;

pub(crate) fn draw_classic_bar<D>(
    lcd: &mut D,
    x: i32,
    base_y: i32,
    level: u8,
    total: u8,
) where
    D: DrawTarget<Color = Rgb565>,
{
    let hollow_start = total.saturating_sub(BAR_HOLLOW_COUNT);

    for i in 0..total {
        if i >= level {
            break;
        }
        let sx = x + i as i32 * BAR_SEG_PITCH;
        let h = (i as i32 + 1).min(BAR_MAX_H);
        let sy = base_y - h + 1;
        let size = Size::new(BAR_SEG_W as u32, h as u32);

        if i < hollow_start {
            Rectangle::new(Point::new(sx, sy), size)
                .into_styled(PrimitiveStyle::with_fill(FG))
                .draw(lcd)
                .ok();
        } else {
            // Outline, not `with_stroke` -- see `icons.rs`'s module doc.
            icons::draw_outline(lcd, Point::new(sx, sy), size, FG);
        }
    }
}

const S_METER_SEGS: u8 = 13;

fn clear_row<D>(lcd: &mut D, row_y: i32)
where
    D: DrawTarget<Color = Rgb565>,
{
    Rectangle::new(
        Point::new(0, row_y),
        Size::new(SCREEN_W as u32, S_METER_HEIGHT as u32),
    )
    .into_styled(PrimitiveStyle::with_fill(BG))
    .draw(lcd)
    .ok();
}

pub(crate) fn draw_s_meter<D>(lcd: &mut D, app: &app::App, row_y: i32)
where
    D: DrawTarget<Color = Rgb565>,
{
    clear_row(lcd, row_y);
    let dbm = app.rssi_dbm();
    let level = app.s_meter_level();
    let over = app.s_meter_over_s9_dbm();

    let mut line: TextBuf<20> = TextBuf::new();
    if over > 0 {
        write!(line, "{}dBm +{}", dbm, over).ok();
    } else {
        write!(line, "{}dBm S{}", dbm, app.s_meter_s_number() as u32).ok();
    }
    Text::new(
        line.as_str(),
        Point::new(4, row_y + FONT_5X8.baseline as i32),
        MonoTextStyle::new(&FONT_5X8, FG),
    )
    .draw(lcd)
    .ok();

    let bar_w = S_METER_SEGS as i32 * BAR_SEG_PITCH;
    let bar_x = SCREEN_W - RIGHT_MARGIN - bar_w;
    let base_y = row_y + S_METER_HEIGHT - 1;

    draw_classic_bar(lcd, bar_x, base_y, level, S_METER_SEGS);
}

const MIC_BAR_SEGS: u8 = 18;

fn draw_mic_bar<D>(lcd: &mut D, app: &app::App, row_y: i32)
where
    D: DrawTarget<Color = Rgb565>,
{
    clear_row(lcd, row_y);
    let mic = app.mic_level();
    let level = ((mic as u16 * MIC_BAR_SEGS as u16 + 127) / 255)
        .min(MIC_BAR_SEGS as u16) as u8;

    Text::new(
        "MIC",
        Point::new(4, row_y + FONT_5X8.baseline as i32),
        MonoTextStyle::new(&FONT_5X8, ALERT),
    )
    .draw(lcd)
    .ok();

    let bar_x = 4 + 3 * FONT_5X8.character_size.width as i32 + 4;
    let base_y = row_y + S_METER_HEIGHT - 1;

    draw_classic_bar(lcd, bar_x, base_y, level, MIC_BAR_SEGS);
}

fn draw_tx_row<D>(lcd: &mut D, app: &app::App, top: i32)
where
    D: DrawTarget<Color = Rgb565>,
{
    icons::draw_antenna(lcd, 4, top, ALERT);

    let pwr = match app.side_power(app.master_index()) {
        Power::Low => 2,
        Power::Mid => 4,
        Power::High => 6,
    };
    for k in 1..=pwr {
        let h = (k + 2).min(10);
        Rectangle::new(
            Point::new(12 + k * 4, top + 10 - h),
            Size::new(3, h as u32),
        )
        .into_styled(PrimitiveStyle::with_fill(ALERT))
        .draw(lcd)
        .ok();
    }
}
