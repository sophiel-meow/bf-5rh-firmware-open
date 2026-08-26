use super::list::{draw_list, Cache, ListSource};
use crate::app;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;

struct SettingsSource<'a, 'b>(&'a app::App<'b>);

impl<'a, 'b> ListSource for SettingsSource<'a, 'b> {
    fn row_count(&mut self) -> usize {
        self.0.settings_item_count()
    }

    fn label(&mut self, index: usize, w: &mut dyn core::fmt::Write) {
        let _ = write!(w, "{}", self.0.settings_label_at(index));
    }

    fn value(&mut self, index: usize, w: &mut dyn core::fmt::Write) -> bool {
        self.0.settings_value_at(index, w)
    }

    fn cursor(&mut self, index: usize) -> Option<usize> {
        self.0.settings_cursor(index)
    }
}

pub fn draw_settings<D>(lcd: &mut D, app: &app::App, cache: &mut Cache)
where
    D: DrawTarget<Color = Rgb565>,
{
    let mut source = SettingsSource(app);
    draw_list(
        lcd,
        app.settings_title(),
        &mut source,
        app.settings_index(),
        app.settings_show_arrows(),
        cache,
    );
}
