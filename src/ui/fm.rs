use super::list::{draw_list, Cache as ListCache, ListSource};
use super::standby::{draw_frequency, draw_right_aligned};
use super::{clear_strip, TextBuf, SCREEN_H};
use crate::app;
use core::fmt::Write as _;
use embedded_graphics::mono_font::{ascii::FONT_6X10, ascii::FONT_9X18, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;

const FG: Rgb565 = Rgb565::WHITE;

const HEADER_Y: i32 = 4;
const HEADER_H: i32 = 12;
const FREQ_Y: i32 = 40;
const FREQ_H: i32 = 22;
const STATUS_Y: i32 = 86;
const STATUS_H: i32 = 14;

#[rustfmt::skip]
const CHANNEL_LABELS: [&str; 30] = [
    "CH 01", "CH 02", "CH 03", "CH 04", "CH 05", "CH 06", "CH 07", "CH 08", "CH 09", "CH 10",
    "CH 11", "CH 12", "CH 13", "CH 14", "CH 15", "CH 16", "CH 17", "CH 18", "CH 19", "CH 20",
    "CH 21", "CH 22", "CH 23", "CH 24", "CH 25", "CH 26", "CH 27", "CH 28", "CH 29", "CH 30",
];

struct FmSaveSource<'a, 'b>(&'a app::App<'b>);

impl<'a, 'b> ListSource for FmSaveSource<'a, 'b> {
    fn row_count(&mut self) -> usize {
        crate::flash_map::FM_CHANNEL_COUNT
    }

    fn label(&mut self, index: usize, w: &mut dyn core::fmt::Write) {
        let _ = write!(w, "{}", CHANNEL_LABELS[index]);
    }

    fn value(&mut self, index: usize, w: &mut dyn core::fmt::Write) -> bool {
        match self.0.fm_channel_freq_at(index) {
            Some(deci_mhz) => {
                let _ = write!(w, "{}.{}", deci_mhz / 10, deci_mhz % 10);
            }
            None => {
                let _ = write!(w, "EMPTY");
            }
        }
        true
    }
}

#[derive(Clone, Copy, PartialEq)]
enum FmView {
    Tuning,
    Picker,
}

pub(crate) struct Cache {
    view: Option<FmView>,
    tuning: Option<TuningSnap>,
}

#[derive(Clone, Copy, PartialEq)]
struct TuningSnap {
    channel_mode: bool,
    channel_index: u8,
    input_len: usize,
    input_digits: [u8; 4],
    deci_mhz: u16,
    seeking: bool,
    rssi: u8,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            view: None,
            tuning: None,
        }
    }
}

fn capture_tuning(app: &app::App) -> TuningSnap {
    let mut input_digits = [0u8; 4];
    for (i, d) in input_digits.iter_mut().enumerate() {
        *d = app.fm_input_digit(i);
    }
    TuningSnap {
        channel_mode: app.fm_is_channel_mode(),
        channel_index: app.fm_channel_index(),
        input_len: app.fm_input_len(),
        input_digits,
        deci_mhz: app.fm_deci_mhz(),
        seeking: app.fm_is_seeking(),
        rssi: app.fm_rssi(),
    }
}

pub fn draw_fm<D>(lcd: &mut D, app: &app::App, cache: &mut Cache, list_cache: &mut ListCache)
where
    D: DrawTarget<Color = Rgb565>,
{
    let view = if app.fm_save_picker_selected().is_some() {
        FmView::Picker
    } else {
        FmView::Tuning
    };

    if cache.view != Some(view) {
        match view {
            FmView::Picker => *list_cache = ListCache::new(),
            FmView::Tuning => cache.tuning = None,
        }
    }
    cache.view = Some(view);

    match view {
        FmView::Picker => draw_save_picker(lcd, app, list_cache),
        FmView::Tuning => draw_tuning(lcd, app, &mut cache.tuning),
    }
}

fn draw_save_picker<D>(lcd: &mut D, app: &app::App, list_cache: &mut ListCache)
where
    D: DrawTarget<Color = Rgb565>,
{
    let mut source = FmSaveSource(app);
    let selected = app.fm_save_picker_selected().unwrap_or(0) as usize;
    draw_list(lcd, "SAVE TO", &mut source, selected, false, list_cache);
}

fn draw_tuning<D>(lcd: &mut D, app: &app::App, tuning: &mut Option<TuningSnap>)
where
    D: DrawTarget<Color = Rgb565>,
{
    let snap = capture_tuning(app);

    match *tuning {
        None => full_redraw_tuning(lcd, app),
        Some(prev) => {
            if (prev.channel_mode, prev.channel_index) != (snap.channel_mode, snap.channel_index) {
                draw_tuning_header(lcd, app);
            }
            if prev.channel_mode != snap.channel_mode
                || prev.input_len != snap.input_len
                || prev.input_digits != snap.input_digits
                || prev.deci_mhz != snap.deci_mhz
            {
                clear_strip(lcd, FREQ_Y, FREQ_H);
                draw_tuning_freq(lcd, app);
            }
            if (prev.seeking, prev.rssi) != (snap.seeking, snap.rssi) {
                draw_tuning_status(lcd, app);
            }
        }
    }

    *tuning = Some(snap);
}

fn full_redraw_tuning<D>(lcd: &mut D, app: &app::App)
where
    D: DrawTarget<Color = Rgb565>,
{
    clear_strip(lcd, 0, SCREEN_H);

    draw_tuning_header(lcd, app);
    draw_tuning_freq(lcd, app);
    draw_tuning_status(lcd, app);

    Text::new(
        "VM CH/VFO  MENU SAVE",
        Point::new(4, 120),
        MonoTextStyle::new(&FONT_6X10, FG),
    )
    .draw(lcd)
    .ok();
}

fn draw_tuning_header<D>(lcd: &mut D, app: &app::App)
where
    D: DrawTarget<Color = Rgb565>,
{
    clear_strip(lcd, HEADER_Y, HEADER_H);
    let mut header: TextBuf<20> = TextBuf::new();
    if app.fm_is_channel_mode() {
        write!(header, "FM CH {:02}", app.fm_channel_index() as u32 + 1).ok();
    } else {
        write!(header, "FM VFO").ok();
    }
    Text::new(
        header.as_str(),
        Point::new(4, 14),
        MonoTextStyle::new(&FONT_6X10, FG),
    )
    .draw(lcd)
    .ok();
}

fn draw_tuning_freq<D>(lcd: &mut D, app: &app::App)
where
    D: DrawTarget<Color = Rgb565>,
{
    if app.fm_input_len() > 0 {
        draw_freq_input(lcd, app, 60);
    } else {
        let freq_hz = app.fm_deci_mhz() as u32 * 100_000;
        draw_frequency(lcd, freq_hz, 60);
    }
}

fn draw_tuning_status<D>(lcd: &mut D, app: &app::App)
where
    D: DrawTarget<Color = Rgb565>,
{
    clear_strip(lcd, STATUS_Y, STATUS_H);
    let mut status: TextBuf<20> = TextBuf::new();
    if app.fm_is_seeking() {
        write!(status, "SEEK...").ok();
    } else {
        write!(status, "RSSI {}", app.fm_rssi() as u32).ok();
    }
    Text::new(
        status.as_str(),
        Point::new(4, 96),
        MonoTextStyle::new(&FONT_6X10, FG),
    )
    .draw(lcd)
    .ok();
}

fn draw_freq_input<D>(lcd: &mut D, app: &app::App, baseline_y: i32)
where
    D: DrawTarget<Color = Rgb565>,
{
    let len = app.fm_input_len();
    let mut buf: TextBuf<8> = TextBuf::new();
    if app.fm_is_channel_mode() {
        for pos in 0..2usize {
            if pos < len {
                write!(buf, "{}", app.fm_input_digit(pos) as u32).ok();
            } else {
                write!(buf, "-").ok();
            }
        }
    } else {
        for pos in 0..4usize {
            if pos == 3 {
                write!(buf, ".").ok();
            }
            if pos < len {
                write!(buf, "{}", app.fm_input_digit(pos) as u32).ok();
            } else {
                write!(buf, "-").ok();
            }
        }
    }
    draw_right_aligned(lcd, buf.as_str(), &FONT_9X18, baseline_y, FG);
}
