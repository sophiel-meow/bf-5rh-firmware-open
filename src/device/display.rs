use crate::board;
use crate::drivers::st7735::St7735;
use at32f421_pac as pac;
use cortex_m::peripheral::SYST;

pub struct Display<'a> {
    lcd: St7735<'a>,
}

impl<'a> Display<'a> {
    pub fn new(gpiob: &'a pac::Gpiob, spi2: &'a pac::Spi2) -> Self {
        Display {
            lcd: St7735::new(gpiob, spi2),
        }
    }

    pub fn init(&mut self, syst: &mut SYST) {
        self.lcd.init(syst);
    }

    pub fn as_draw_target(&mut self) -> &mut St7735<'a> {
        &mut self.lcd
    }
}

pub struct Backlight<'a> {
    gpioa: &'a pac::Gpioa,
}

impl<'a> Backlight<'a> {
    pub fn new(gpioa: &'a pac::Gpioa) -> Self {
        Backlight { gpioa }
    }

    pub fn on(&self) {
        board::set_lcd_backlight(self.gpioa, true);
    }

    pub fn off(&self) {
        board::set_lcd_backlight(self.gpioa, false);
    }
}
