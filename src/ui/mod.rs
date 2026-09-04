pub(crate) mod boot;
pub(crate) mod cps;
mod icons;
mod launcher;
mod list;
mod scan;
mod standby;

pub(crate) use list::{draw_list, Cache, ListSource};

use crate::app;
use crate::device::display::Display;
use cortex_m::peripheral::SYST;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};

pub(crate) const SCREEN_W: i32 = crate::drivers::st7735::WIDTH as i32;
pub(crate) const SCREEN_H: i32 = crate::drivers::st7735::HEIGHT as i32;

pub(crate) fn clear_strip<D>(lcd: &mut D, y: i32, h: i32)
where
    D: DrawTarget<Color = Rgb565>,
{
    Rectangle::new(Point::new(0, y), Size::new(SCREEN_W as u32, h as u32))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(lcd)
        .ok();
}

#[derive(Clone, Copy, PartialEq)]
pub struct TextBuf<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> TextBuf<N> {
    pub fn new() -> Self {
        TextBuf {
            buf: [0; N],
            len: 0,
        }
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[..self.len]).unwrap_or("")
    }
}

impl<const N: usize> core::fmt::Write for TextBuf<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let space = N - self.len;
        let n = bytes.len().min(space);
        self.buf[self.len..self.len + n].copy_from_slice(&bytes[..n]);
        self.len += n;
        Ok(())
    }
}

pub struct UiState {
    last_mode: Option<app::Mode>,
    standby: standby::Cache,
    list: list::Cache,
    scan: scan::Cache,
}

impl UiState {
    pub fn new() -> Self {
        UiState {
            last_mode: None,
            standby: standby::Cache::new(),
            list: list::Cache::new(),
            scan: scan::Cache::new(),
        }
    }
}

#[allow(dead_code)]
fn draw_stack_probe<D>(lcd: &mut D)
where
    D: DrawTarget<Color = Rgb565>,
{
    use core::fmt::Write;
    use embedded_graphics::mono_font::{ascii::FONT_6X10, MonoTextStyle};
    use embedded_graphics::text::Text;

    let (used, free) = crate::hal::stackprobe::high_water();
    let mut t: TextBuf<16> = TextBuf::new();
    let _ = write!(t, "U{used} F{free}");

    let y = SCREEN_H - 10;
    clear_strip(lcd, y, 10);
    Text::new(
        t.as_str(),
        Point::new(1, y + 8),
        MonoTextStyle::new(&FONT_6X10, Rgb565::new(31, 63, 0)),
    )
    .draw(lcd)
    .ok();
}

#[inline(never)]
pub fn draw(
    display: &mut Display<'_>,
    app: &mut app::App<'_>,
    state: &mut UiState,
    syst: &mut SYST,
) {
    let lcd = display.as_draw_target();
    let mode = app.mode();
    if state.last_mode != Some(mode) {
        state.standby = standby::Cache::new();
        state.list = list::Cache::new();
        state.scan = scan::Cache::new();
    }
    state.last_mode = Some(mode);
    match mode {
        app::Mode::AppMenu => {
            launcher::draw_app_menu(lcd, app, &mut state.list)
        }
        app::Mode::Scan => scan::draw_scan(lcd, app, &mut state.scan),
        app::Mode::External(_) => app::overlay::draw(lcd, app, syst),
        _ => standby::draw_standby(lcd, app, &mut state.standby),
    };
}
