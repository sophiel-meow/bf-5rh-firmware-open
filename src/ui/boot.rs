use super::{TextBuf, SCREEN_H, SCREEN_W};
use crate::flash_map::{self, Settings};
use core::fmt::Write as _;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::{
    ascii::{FONT_6X10, FONT_9X18},
    MonoTextStyle,
};
use embedded_graphics::pixelcolor::{raw::RawU16, Rgb565};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::Text;

const BG: Rgb565 = Rgb565::BLACK;
const FG: Rgb565 = Rgb565::WHITE;
const ACCENT: Rgb565 = Rgb565::YELLOW;

pub fn clear<D>(lcd: &mut D)
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
}

fn ascii_line(raw: &[u8; 16]) -> &str {
    let end = raw
        .iter()
        .position(|&b| b == 0x00 || b == 0xFF)
        .unwrap_or(raw.len());
    core::str::from_utf8(&raw[..end]).unwrap_or("")
}

fn draw_centered_big<D>(lcd: &mut D, text: &str, baseline_y: i32, color: Rgb565)
where
    D: DrawTarget<Color = Rgb565>,
{
    let width =
        text.chars().count() as i32 * FONT_9X18.character_size.width as i32;
    let x = ((SCREEN_W - width) / 2).max(0);
    Text::new(
        text,
        Point::new(x, baseline_y),
        MonoTextStyle::new(&FONT_9X18, color),
    )
    .draw(lcd)
    .ok();
}

fn draw_centered_small<D>(
    lcd: &mut D,
    text: &str,
    baseline_y: i32,
    color: Rgb565,
) where
    D: DrawTarget<Color = Rgb565>,
{
    let width =
        text.chars().count() as i32 * FONT_6X10.character_size.width as i32;
    let x = ((SCREEN_W - width) / 2).max(0);
    Text::new(
        text,
        Point::new(x, baseline_y),
        MonoTextStyle::new(&FONT_6X10, color),
    )
    .draw(lcd)
    .ok();
}

fn text_lines(settings: &Settings) -> (&str, &str) {
    let l1 = ascii_line(&settings.boot_text_line1);
    let l2 = ascii_line(&settings.boot_text_line2);
    if l1.is_empty() && l2.is_empty() {
        ("UV-5RH Sylph", "73 DE BG4KNN")
    } else {
        (l1, l2)
    }
}

pub fn draw_voltage<D>(lcd: &mut D, voltage_cv: u16)
where
    D: DrawTarget<Color = Rgb565>,
{
    clear(lcd);
    let mut line: TextBuf<16> = TextBuf::new();
    write!(line, "{}.{:02}V", voltage_cv / 100, voltage_cv % 100).ok();
    draw_centered_small(lcd, "BATTERY", 44, ACCENT);
    draw_centered_big(lcd, line.as_str(), 72, FG);
}

pub fn draw_message<D>(lcd: &mut D, settings: &Settings)
where
    D: DrawTarget<Color = Rgb565>,
{
    clear(lcd);
    let (l1, l2) = text_lines(settings);
    draw_centered_big(lcd, l1, 52, FG);
    draw_centered_small(lcd, l2, 76, ACCENT);
}

pub const LOGO_CHUNK_ROWS: u16 = 4;
pub const LOGO_CHUNK_BYTES: usize =
    flash_map::BOOT_LOGO_WIDTH as usize * LOGO_CHUNK_ROWS as usize * 2;

pub fn draw_logo_chunk<D>(lcd: &mut D, y0: u16, chunk: &[u8])
where
    D: DrawTarget<Color = Rgb565>,
{
    let area = Rectangle::new(
        Point::new(0, y0 as i32),
        Size::new(flash_map::BOOT_LOGO_WIDTH as u32, LOGO_CHUNK_ROWS as u32),
    );
    let pixels = chunk
        .chunks_exact(2)
        .map(|b| Rgb565::from(RawU16::new(u16::from_le_bytes([b[0], b[1]]))));
    lcd.fill_contiguous(&area, pixels).ok();
}
