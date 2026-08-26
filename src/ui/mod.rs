mod chanmgr;
mod fm;
mod icons;
mod launcher;
mod list;
mod scan;
mod scanqt;
mod search;
mod settings;
mod standby;

use crate::app;
use crate::device::display::Display;
use crate::device::radio::SubAudio;
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

pub(crate) fn tone_text<W: core::fmt::Write>(w: &mut W, tone: Option<SubAudio>) {
    match tone {
        None | Some(SubAudio::None) => {
            let _ = write!(w, "NONE");
        }
        Some(SubAudio::Ctcss(hz)) => {
            let _ = write!(w, "{}.{}Hz", hz / 10, hz % 10);
        }
        Some(SubAudio::Dcs { code, inverted }) => {
            let _ = write!(w, "D{:03o}{}", code, if inverted { "I" } else { "N" });
        }
    }
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
    scanqt: scanqt::Cache,
    search: search::Cache,
    fm: fm::Cache,
}

impl UiState {
    pub fn new() -> Self {
        UiState {
            last_mode: None,
            standby: standby::Cache::new(),
            list: list::Cache::new(),
            scan: scan::Cache::new(),
            scanqt: scanqt::Cache::new(),
            search: search::Cache::new(),
            fm: fm::Cache::new(),
        }
    }
}

pub fn draw(display: &mut Display<'_>, app: &mut app::App<'_>, state: &mut UiState) {
    let lcd = display.as_draw_target();
    let mode = app.mode();
    if state.last_mode != Some(mode) {
        state.standby = standby::Cache::new();
        state.list = list::Cache::new();
        state.scan = scan::Cache::new();
        state.scanqt = scanqt::Cache::new();
        state.search = search::Cache::new();
        state.fm = fm::Cache::new();
    }
    state.last_mode = Some(mode);
    match mode {
        app::Mode::AppMenu => launcher::draw_app_menu(lcd, app, &mut state.list),
        app::Mode::Settings => settings::draw_settings(lcd, app, &mut state.list),
        app::Mode::ChanMgr => chanmgr::draw_chanmgr(lcd, app, &mut state.list),
        app::Mode::Scan => scan::draw_scan(lcd, app, &mut state.scan),
        app::Mode::Search => search::draw_search(lcd, app, &mut state.search),
        app::Mode::ScanQt => scanqt::draw_scanqt(lcd, app, &mut state.scanqt),
        app::Mode::Fm => fm::draw_fm(lcd, app, &mut state.fm, &mut state.list),
        _ => standby::draw_standby(lcd, app, &mut state.standby),
    };
}
