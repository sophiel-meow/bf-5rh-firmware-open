use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};

pub fn draw_outline<D>(
    target: &mut D,
    top_left: Point,
    size: Size,
    color: Rgb565,
) where
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

pub const BATTERY_W: i32 = 22;
pub const BATTERY_H: i32 = 10;
const BATTERY_BODY_W: i32 = 20;
const BATTERY_NUB_W: i32 = 2;
const BATTERY_SEGMENTS: i32 = 4;

pub fn draw_battery<D>(
    target: &mut D,
    x: i32,
    y: i32,
    bars: u8,
    shell: Rgb565,
    fill: Rgb565,
) where
    D: DrawTarget<Color = Rgb565>,
{
    draw_outline(
        target,
        Point::new(x, y),
        Size::new(BATTERY_BODY_W as u32, BATTERY_H as u32),
        shell,
    );
    Rectangle::new(
        Point::new(x + BATTERY_BODY_W, y + 3),
        Size::new(BATTERY_NUB_W as u32, 4),
    )
    .into_styled(PrimitiveStyle::with_fill(shell))
    .draw(target)
    .ok();

    let seg_w = (BATTERY_BODY_W - 4) / BATTERY_SEGMENTS;
    for slot in 0..bars.min(BATTERY_SEGMENTS as u8) as i32 {
        Rectangle::new(
            Point::new(x + 2 + slot * seg_w, y + 2),
            Size::new((seg_w - 1) as u32, (BATTERY_H - 4) as u32),
        )
        .into_styled(PrimitiveStyle::with_fill(fill))
        .draw(target)
        .ok();
    }
}

pub fn draw_padlock<D>(target: &mut D, x: i32, y: i32, color: Rgb565)
where
    D: DrawTarget<Color = Rgb565>,
{
    let fill = PrimitiveStyle::with_fill(color);
    let mut rect = |rx: i32, ry: i32, w: u32, h: u32| {
        Rectangle::new(Point::new(x + rx, y + ry), Size::new(w, h))
            .into_styled(fill)
            .draw(target)
            .ok();
    };
    rect(1, 0, 5, 1); // shackle
    rect(0, 1, 1, 3);
    rect(6, 1, 1, 3);
    rect(0, 4, 7, 5); // body
}
