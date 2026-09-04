use embedded_graphics::mono_font::{ascii::FONT_9X18, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::Text;

pub fn draw_programming<D>(target: &mut D)
where
    D: DrawTarget<Color = Rgb565>,
{
    Rectangle::new(Point::new(0, 0), Size::new(160, 128))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(target)
        .ok();

    let style = MonoTextStyle::new(&FONT_9X18, Rgb565::WHITE);
    Text::new("CPS", Point::new(64, 40), style)
        .draw(target)
        .ok();
    Text::new("PROGRAMMING", Point::new(16, 72), style)
        .draw(target)
        .ok();
}
