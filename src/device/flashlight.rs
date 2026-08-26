use crate::board;
use at32f421_pac as pac;

pub struct Flashlight<'a> {
    gpiof: &'a pac::Gpiof,
    on: bool,
}

impl<'a> Flashlight<'a> {
    pub fn new(gpiof: &'a pac::Gpiof) -> Self {
        board::init_flashlight_pin(gpiof);
        Flashlight { gpiof, on: false }
    }

    pub fn toggle(&mut self) {
        self.on = !self.on;
        board::set_flashlight(self.gpiof, self.on);
    }
}
