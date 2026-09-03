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
/// Header band + "Sylph" wordmark.
const ACCENT: Rgb565 = Rgb565::new(0, 55, 28);
const HEADER_BG: Rgb565 = Rgb565::new(2, 6, 12);
/// Second custom line.
const AMBER: Rgb565 = Rgb565::new(31, 50, 0);
/// Footer text.
const DIM: Rgb565 = Rgb565::new(13, 26, 13);
/// Hairlines flanking the footer and the second custom line.
const HAIR: Rgb565 = Rgb565::new(6, 12, 6);
const GAUGE_OK: Rgb565 = Rgb565::new(0, 55, 6);
const GAUGE_WARN: Rgb565 = Rgb565::new(31, 50, 0);
const GAUGE_LOW: Rgb565 = Rgb565::new(31, 8, 4);

const BIG_W: i32 = FONT_9X18.character_size.width as i32;
const SMALL_W: i32 = FONT_6X10.character_size.width as i32;

fn fill<D>(lcd: &mut D, x: i32, y: i32, w: i32, h: i32, color: Rgb565)
where
    D: DrawTarget<Color = Rgb565>,
{
    if w <= 0 || h <= 0 {
        return;
    }
    Rectangle::new(Point::new(x, y), Size::new(w as u32, h as u32))
        .into_styled(PrimitiveStyle::with_fill(color))
        .draw(lcd)
        .ok();
}

pub fn clear<D>(lcd: &mut D)
where
    D: DrawTarget<Color = Rgb565>,
{
    fill(lcd, 0, 0, SCREEN_W, SCREEN_H, BG);
}

fn text_big<D>(lcd: &mut D, s: &str, x: i32, baseline: i32, color: Rgb565)
where
    D: DrawTarget<Color = Rgb565>,
{
    Text::new(
        s,
        Point::new(x, baseline),
        MonoTextStyle::new(&FONT_9X18, color),
    )
    .draw(lcd)
    .ok();
}

fn text_small<D>(lcd: &mut D, s: &str, x: i32, baseline: i32, color: Rgb565)
where
    D: DrawTarget<Color = Rgb565>,
{
    Text::new(
        s,
        Point::new(x, baseline),
        MonoTextStyle::new(&FONT_6X10, color),
    )
    .draw(lcd)
    .ok();
}

fn centered(text: &str, char_w: i32) -> i32 {
    ((SCREEN_W - text.len() as i32 * char_w) / 2).max(0)
}

fn ascii_line(raw: &[u8; 16]) -> &str {
    let end = raw
        .iter()
        .position(|&b| b == 0x00 || b == 0xFF)
        .unwrap_or(raw.len());
    core::str::from_utf8(&raw[..end]).unwrap_or("")
}

fn text_lines(settings: &Settings) -> (&str, &str) {
    let l1 = ascii_line(&settings.boot_text_line1);
    let l2 = ascii_line(&settings.boot_text_line2);
    if l1.is_empty() && l2.is_empty() {
        ("WELCOME", "73 DE BG4KNN")
    } else {
        (l1, l2)
    }
}

fn draw_header<D>(lcd: &mut D)
where
    D: DrawTarget<Color = Rgb565>,
{
    const MODEL: &str = "UV-5RH";
    const FIRMWARE: &str = "Sylph";
    fill(lcd, 0, 0, SCREEN_W, 26, HEADER_BG);
    fill(lcd, 0, 26, SCREEN_W, 2, ACCENT);
    text_big(lcd, MODEL, 7, 19, FG);
    text_big(
        lcd,
        FIRMWARE,
        SCREEN_W - 7 - FIRMWARE.len() as i32 * BIG_W,
        19,
        ACCENT,
    );
}

fn draw_lines<D>(lcd: &mut D, settings: &Settings)
where
    D: DrawTarget<Color = Rgb565>,
{
    let (l1, l2) = text_lines(settings);
    text_big(lcd, l1, centered(l1, BIG_W), 52, FG);

    let x2 = centered(l2, SMALL_W);
    text_small(lcd, l2, x2, 72, AMBER);
    let rule_y = 68;
    fill(lcd, 8, rule_y, x2 - 14, 1, HAIR);
    let right = x2 + l2.len() as i32 * SMALL_W + 6;
    fill(lcd, right, rule_y, SCREEN_W - 8 - right, 1, HAIR);
}

const GAUGE_Y: i32 = 97;
const GAUGE_W: i32 = 63;
const GAUGE_SEGMENTS: i32 = 4;

fn draw_battery<D>(lcd: &mut D, voltage_cv: u16, bars: u8)
where
    D: DrawTarget<Color = Rgb565>,
{
    let mut line: TextBuf<8> = TextBuf::new();
    write!(line, "{}.{:02}V", voltage_cv / 100, voltage_cv % 100).ok();
    let line = line.as_str();
    text_big(lcd, line, centered(line, BIG_W), GAUGE_Y, FG);

    let bars = bars.min(GAUGE_SEGMENTS as u8) as i32;
    let color = match bars {
        0 | 1 => GAUGE_LOW,
        2 => GAUGE_WARN,
        _ => GAUGE_OK,
    };
    let x0 = (SCREEN_W - GAUGE_W) / 2;
    let seg_w = (GAUGE_W + 1) / GAUGE_SEGMENTS - 1;
    for i in 0..GAUGE_SEGMENTS {
        fill(
            lcd,
            x0 + i * (seg_w + 1),
            GAUGE_Y + 7,
            seg_w,
            2,
            if i < bars { color } else { HAIR },
        );
    }
}

const FOOTER_RULE_Y: i32 = 110;
const FOOTER_BASELINE: i32 = 120;

fn draw_footer<D>(lcd: &mut D, chip_id: u16)
where
    D: DrawTarget<Color = Rgb565>,
{
    fill(lcd, 6, FOOTER_RULE_Y, SCREEN_W - 12, 1, HAIR);

    let mut chip: TextBuf<12> = TextBuf::new();
    // match chip_id {
    //     0x4819 => write!(chip, "FD6818"),
    //     0x4829 => write!(chip, "FD6818B"),
    //     other => write!(chip, "RFIC {other:04X}"),
    // }
    //     .ok();
    write!(chip, "0x{chip_id:04X}").ok();
    text_small(lcd, chip.as_str(), 7, FOOTER_BASELINE, DIM);

    let ver = crate::app::FIRMWARE_VERSION;
    let max = 12usize;
    let ver = &ver[ver.len().saturating_sub(max)..];
    text_small(
        lcd,
        ver,
        SCREEN_W - 7 - ver.len() as i32 * SMALL_W,
        FOOTER_BASELINE,
        DIM,
    );
}

pub fn draw_message<D>(
    lcd: &mut D,
    settings: &Settings,
    voltage_cv: u16,
    bars: u8,
    chip_id: u16,
) where
    D: DrawTarget<Color = Rgb565>,
{
    clear(lcd);
    draw_header(lcd);
    draw_lines(lcd, settings);
    draw_battery(lcd, voltage_cv, bars);
    draw_footer(lcd, chip_id);
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
