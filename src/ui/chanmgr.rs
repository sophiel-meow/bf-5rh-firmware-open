use super::list::{draw_list, Cache, ListSource};
use super::TextBuf;
use crate::app;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;

struct ChanListSource<'a, 'b>(&'a mut app::App<'b>);

impl<'a, 'b> ListSource for ChanListSource<'a, 'b> {
    fn row_count(&mut self) -> usize {
        self.0.chanmgr_list_row_count()
    }

    fn label(&mut self, index: usize, w: &mut dyn core::fmt::Write) {
        self.0.chanmgr_list_label(index, w);
    }
}

struct ChanDetailSource<'a, 'b>(&'a app::App<'b>);

impl<'a, 'b> ListSource for ChanDetailSource<'a, 'b> {
    fn row_count(&mut self) -> usize {
        self.0.chanmgr_field_count()
    }

    fn label(&mut self, index: usize, w: &mut dyn core::fmt::Write) {
        self.0.chanmgr_field_label(index, w);
    }

    fn value(&mut self, index: usize, w: &mut dyn core::fmt::Write) -> bool {
        self.0.chanmgr_field_value(index, w)
    }

    fn cursor(&mut self, index: usize) -> Option<usize> {
        self.0.chanmgr_field_cursor(index)
    }
}

pub fn draw_chanmgr<D>(lcd: &mut D, app: &mut app::App, cache: &mut Cache)
where
    D: DrawTarget<Color = Rgb565>,
{
    if app.chanmgr_is_detail() {
        let mut title: TextBuf<16> = TextBuf::new();
        app.chanmgr_detail_title(&mut title);
        let selected = app.chanmgr_field_index();
        let show_arrows = app.chanmgr_show_arrows();
        let mut source = ChanDetailSource(app);
        draw_list(lcd, title.as_str(), &mut source, selected, show_arrows, cache);
    } else {
        let selected = app.chanmgr_list_selected_index();
        let mut source = ChanListSource(app);
        draw_list(lcd, "CHANNELS", &mut source, selected, false, cache);
    }
}
