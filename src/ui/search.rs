use super::standby::draw_frequency;
use super::{clear_strip, tone_text, TextBuf, SCREEN_H};
use crate::app::{self, SearchStatus};
use crate::device::radio::SubAudio;
use core::fmt::Write as _;
use embedded_graphics::mono_font::{ascii::FONT_6X10, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;

const FG: Rgb565 = Rgb565::WHITE;

const HEADER_Y: i32 = 4;
const HEADER_H: i32 = 12;
const CONTENT_Y: i32 = 32;
const CONTENT_H: i32 = 68;

#[derive(Clone, Copy, PartialEq)]
struct Snapshot {
    band: &'static str,
    status: SearchStatus,
    freq_hz: u32,
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

pub fn draw_search<D>(lcd: &mut D, app: &app::App, cache: &mut Cache)
where
    D: DrawTarget<Color = Rgb565>,
{
    let snap = Snapshot {
        band: app.search_band_label(),
        status: app.search_status(),
        freq_hz: app.search_candidate_freq_hz(),
        tone: app.search_tone(),
    };

    match cache.snap {
        None => full_redraw(lcd, app),
        Some(prev) => {
            if prev.band != snap.band {
                draw_header(lcd, app);
            }
            if (prev.status, prev.freq_hz, prev.tone) != (snap.status, snap.freq_hz, snap.tone) {
                draw_content(lcd, app);
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

    draw_header(lcd, app);
    draw_content(lcd, app);

    Text::new(
        "AB BAND  MENU SAVE",
        Point::new(4, 120),
        MonoTextStyle::new(&FONT_6X10, FG),
    )
    .draw(lcd)
    .ok();
}

fn draw_header<D>(lcd: &mut D, app: &app::App)
where
    D: DrawTarget<Color = Rgb565>,
{
    clear_strip(lcd, HEADER_Y, HEADER_H);
    let mut header: TextBuf<20> = TextBuf::new();
    write!(header, "SEARCH {}", app.search_band_label()).ok();
    Text::new(
        header.as_str(),
        Point::new(4, 14),
        MonoTextStyle::new(&FONT_6X10, FG),
    )
    .draw(lcd)
    .ok();
}

fn draw_content<D>(lcd: &mut D, app: &app::App)
where
    D: DrawTarget<Color = Rgb565>,
{
    clear_strip(lcd, CONTENT_Y, CONTENT_H);
    let small = MonoTextStyle::new(&FONT_6X10, FG);

    match app.search_status() {
        SearchStatus::Hunting => {
            Text::new("HUNTING...", Point::new(4, 60), small)
                .draw(lcd)
                .ok();
        }
        SearchStatus::Listening => {
            draw_frequency(lcd, app.search_candidate_freq_hz(), 60);
            Text::new("LISTEN", Point::new(4, 96), small).draw(lcd).ok();
        }
        SearchStatus::Found => {
            draw_frequency(lcd, app.search_candidate_freq_hz(), 50);
            let mut tone_line: TextBuf<16> = TextBuf::new();
            tone_text(&mut tone_line, app.search_tone());
            Text::new(tone_line.as_str(), Point::new(4, 96), small)
                .draw(lcd)
                .ok();
        }
    }
}
