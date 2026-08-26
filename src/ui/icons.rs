use embedded_graphics::mono_font::{ascii::FONT_6X10, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::Text;

pub fn draw_outline<D>(target: &mut D, top_left: Point, size: Size, color: Rgb565)
where
    D: DrawTarget<Color = Rgb565>,
{
    let w = size.width as i32;
    let h = size.height as i32;
    let fill = PrimitiveStyle::with_fill(color);
    Rectangle::new(top_left, Size::new(size.width, 1))
        .into_styled(fill)
        .draw(target)
        .ok();
    Rectangle::new(top_left + Point::new(0, h - 1), Size::new(size.width, 1))
        .into_styled(fill)
        .draw(target)
        .ok();
    Rectangle::new(top_left, Size::new(1, size.height))
        .into_styled(fill)
        .draw(target)
        .ok();
    Rectangle::new(top_left + Point::new(w - 1, 0), Size::new(1, size.height))
        .into_styled(fill)
        .draw(target)
        .ok();
}

const BATTERY_W: i32 = 20;
const BATTERY_H: i32 = 10;
const BATTERY_NUB_W: i32 = 2;
const BATTERY_SEGMENTS: i32 = 4;

pub fn draw_battery<D>(target: &mut D, right_x: i32, y: i32, level: u8, color: Rgb565)
where
    D: DrawTarget<Color = Rgb565>,
{
    let x = right_x - BATTERY_W - BATTERY_NUB_W;
    draw_outline(
        target,
        Point::new(x, y),
        Size::new(BATTERY_W as u32, BATTERY_H as u32),
        color,
    );
    Rectangle::new(
        Point::new(x + BATTERY_W, y + BATTERY_H / 4),
        Size::new(BATTERY_NUB_W as u32, (BATTERY_H / 2) as u32),
    )
    .into_styled(PrimitiveStyle::with_fill(color))
    .draw(target)
    .ok();

    let bars = level.saturating_sub(2).min(4) as i32;
    let seg_w = (BATTERY_W - 4) / BATTERY_SEGMENTS;
    for slot in (BATTERY_SEGMENTS - bars)..BATTERY_SEGMENTS {
        let fx = x + 2 + slot * seg_w;
        Rectangle::new(
            Point::new(fx, y + 2),
            Size::new((seg_w - 1) as u32, (BATTERY_H - 4) as u32),
        )
        .into_styled(PrimitiveStyle::with_fill(color))
        .draw(target)
        .ok();
    }
}

pub fn draw_antenna<D>(target: &mut D, x: i32, y: i32, color: Rgb565)
where
    D: DrawTarget<Color = Rgb565>,
{
    let fill = PrimitiveStyle::with_fill(color);
    Rectangle::new(Point::new(x + 2, y), Size::new(1, 8))
        .into_styled(fill)
        .draw(target)
        .ok();
    Rectangle::new(Point::new(x, y), Size::new(5, 1))
        .into_styled(fill)
        .draw(target)
        .ok();
}

pub fn draw_vfo_marker<D>(target: &mut D, cx: i32, cy: i32, filled: bool, color: Rgb565)
where
    D: DrawTarget<Color = Rgb565>,
{
    let r = 4;
    let top_left = Point::new(cx - r, cy - r);
    let size = Size::new((r * 2) as u32, (r * 2) as u32);
    if filled {
        Rectangle::new(top_left, size)
            .into_styled(PrimitiveStyle::with_fill(color))
            .draw(target)
            .ok();
    } else {
        draw_outline(target, top_left, size, color);
    }
}

pub fn draw_key_lock<D>(target: &mut D, x: i32, y: i32, color: Rgb565)
where
    D: DrawTarget<Color = Rgb565>,
{
    Text::new(
        "LOCK",
        Point::new(x, y + FONT_6X10.baseline as i32),
        MonoTextStyle::new(&FONT_6X10, color),
    )
    .draw(target)
    .ok();
}
