use super::standby::draw_frequency;
use super::{clear_strip, tone_text, TextBuf, SCREEN_H};
use crate::app;
use crate::device::radio::SubAudio;
use embedded_graphics::mono_font::{ascii::FONT_6X10, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;

const FG: Rgb565 = Rgb565::WHITE;

const FREQ_Y: i32 = 40;
const FREQ_H: i32 = 22;
const STATUS_Y: i32 = 86;
const STATUS_H: i32 = 14;

#[derive(Clone, Copy, PartialEq)]
struct Snapshot {
    freq_hz: u32,
    found: bool,
    listening: bool,
    tone: Option<SubAudio>,
}

pub(crate) struct Cache {
    snap: Option<Snapshot>,
}

impl Cache {
    pub fn new() -> Self {
        Cache { snap: None }
    }
}

pub fn draw_scanqt<D>(lcd: &mut D, app: &app::App, cache: &mut Cache)
where
    D: DrawTarget<Color = Rgb565>,
{
    let snap = Snapshot {
        freq_hz: app.watching_freq_hz(),
        found: app.scanqt_is_found(),
        listening: app.scanqt_is_listening(),
        tone: app.scanqt_tone(),
    };

    match cache.snap {
        None => full_redraw(lcd, app),
        Some(prev) => {
            if prev.freq_hz != snap.freq_hz {
                clear_strip(lcd, FREQ_Y, FREQ_H);
                draw_frequency(lcd, snap.freq_hz, 60);
            }
            if (prev.found, prev.listening, prev.tone)
                != (snap.found, snap.listening, snap.tone)
            {
                draw_status_line(lcd, app);
            }
        }
    }

    cache.snap = Some(snap);
}

fn full_redraw<D>(lcd: &mut D, app: &app::App)
where
    D: DrawTarget<Color = Rgb565>,
{
    clear_strip(lcd, 0, SCREEN_H);

    let small = MonoTextStyle::new(&FONT_6X10, FG);
    Text::new("QT SCAN", Point::new(4, 14), small)
        .draw(lcd)
        .ok();

    draw_frequency(lcd, app.watching_freq_hz(), 60);
    draw_status_line(lcd, app);

    Text::new("MENU SAVE  EXIT", Point::new(4, 120), small)
        .draw(lcd)
        .ok();
}

fn draw_status_line<D>(lcd: &mut D, app: &app::App)
where
    D: DrawTarget<Color = Rgb565>,
{
    clear_strip(lcd, STATUS_Y, STATUS_H);
    let small = MonoTextStyle::new(&FONT_6X10, FG);

    let status = if app.scanqt_is_found() {
        "FOUND"
    } else if app.scanqt_is_listening() {
        "DETECTING..."
    } else {
        "WAITING..."
    };
    Text::new(status, Point::new(4, 96), small).draw(lcd).ok();

    if app.scanqt_is_found() {
        let mut tone_line: TextBuf<16> = TextBuf::new();
        tone_text(&mut tone_line, app.scanqt_tone());
        Text::new(tone_line.as_str(), Point::new(90, 96), small)
            .draw(lcd)
            .ok();
    }
}
