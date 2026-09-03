use super::standby::draw_frequency;
use super::{clear_strip, TextBuf, SCREEN_H};
use crate::app;
use core::fmt::Write as _;
use embedded_graphics::mono_font::{ascii::FONT_6X10, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;

const FG: Rgb565 = Rgb565::WHITE;

const HEADER_Y: i32 = 4;
const HEADER_H: i32 = 12;
const FREQ_Y: i32 = 40;
const FREQ_H: i32 = 22;

#[derive(Clone, Copy, PartialEq)]
struct Snapshot {
    dir_up: bool,
    channel_mode: bool,
    channel_num: u16,
    freq_hz: u32,
}

pub(crate) struct Cache {
    snap: Option<Snapshot>,
}

impl Cache {
    pub fn new() -> Self {
        Cache { snap: None }
    }
}

pub fn draw_scan<D>(lcd: &mut D, app: &app::App, cache: &mut Cache)
where
    D: DrawTarget<Color = Rgb565>,
{
    let snap = Snapshot {
        dir_up: app.scan_direction_up(),
        channel_mode: app.watching_is_channel_mode(),
        channel_num: app.watching_channel_num(),
        freq_hz: app.watching_freq_hz(),
    };

    match cache.snap {
        None => full_redraw(lcd, app),
        Some(prev) => {
            if (prev.dir_up, prev.channel_mode, prev.channel_num)
                != (snap.dir_up, snap.channel_mode, snap.channel_num)
            {
                draw_header(lcd, app);
            }
            if prev.freq_hz != snap.freq_hz {
                clear_strip(lcd, FREQ_Y, FREQ_H);
                draw_frequency(lcd, snap.freq_hz, 60, FG);
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
    draw_frequency(lcd, app.watching_freq_hz(), 60, FG);

    Text::new(
        "UP/DN STEP  #/EXIT STOP",
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
    let dir = if app.scan_direction_up() { "UP" } else { "DN" };
    let mut header: TextBuf<20> = TextBuf::new();
    if app.watching_is_channel_mode() {
        write!(header, "SCAN {} M{}", dir, app.watching_channel_num()).ok();
    } else {
        write!(header, "SCAN {}", dir).ok();
    }
    Text::new(
        header.as_str(),
        Point::new(4, 14),
        MonoTextStyle::new(&FONT_6X10, FG),
    )
    .draw(lcd)
    .ok();
}
