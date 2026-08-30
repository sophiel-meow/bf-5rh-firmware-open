use crate::board;
use crate::hal::delay;
use at32f421_pac as pac;
use cortex_m::peripheral::SYST;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Size},
    pixelcolor::{raw::RawU16, Rgb565},
    prelude::*,
    primitives::Rectangle,
    Pixel,
};

pub const WIDTH: u16 = 160;
pub const HEIGHT: u16 = 128;

pub struct St7735<'a> {
    gpiob: &'a pac::Gpiob,
    spi2: &'a pac::Spi2,
}

impl<'a> St7735<'a> {
    pub fn new(gpiob: &'a pac::Gpiob, spi2: &'a pac::Spi2) -> Self {
        St7735 { gpiob, spi2 }
    }

    fn spi_write_byte(&self, b: u8) {
        while self.spi2.sts().read().tdbe().bit_is_clear() {}
        self.spi2.dt().write(|w| unsafe { w.dt().bits(b as u16) });
    }

    fn spi_wait_idle(&self) {
        while self.spi2.sts().read().bf().bit_is_set() {}
    }

    fn write_command(&self, cmd: u8) {
        board::set_lcd_dc(self.gpiob, false);
        board::set_lcd_cs(self.gpiob, false);
        self.spi_write_byte(cmd);
        self.spi_wait_idle();
        board::set_lcd_cs(self.gpiob, true);
    }

    fn write_data(&self, bytes: &[u8]) {
        board::set_lcd_dc(self.gpiob, true);
        board::set_lcd_cs(self.gpiob, false);
        for &b in bytes {
            self.spi_write_byte(b);
        }
        self.spi_wait_idle();
        board::set_lcd_cs(self.gpiob, true);
    }

    fn command(&self, cmd: u8, args: &[u8]) {
        self.write_command(cmd);
        if !args.is_empty() {
            self.write_data(args);
        }
    }

    fn set_window(&self, x0: u16, y0: u16, x1: u16, y1: u16) {
        self.command(
            0x2A,
            &[(x0 >> 8) as u8, x0 as u8, (x1 >> 8) as u8, x1 as u8],
        ); // CASET
        self.command(
            0x2B,
            &[(y0 >> 8) as u8, y0 as u8, (y1 >> 8) as u8, y1 as u8],
        ); // RASET
    }

    fn stream_pixel(&self, color: Rgb565) {
        let raw: u16 = RawU16::from(color).into_inner();
        self.spi_write_byte((raw >> 8) as u8);
        self.spi_write_byte(raw as u8);
    }

    pub fn init(&self, syst: &mut SYST) {
        board::set_lcd_reset(self.gpiob, true);
        delay::ms(syst, 10);
        board::set_lcd_reset(self.gpiob, false);
        delay::ms(syst, 10);
        board::set_lcd_reset(self.gpiob, true);
        delay::ms(syst, 120);

        self.command(0x11, &[]); // SLPOUT
        delay::ms(syst, 120);

        self.command(0xB1, &[0x05, 0x3A, 0x3A]); // FRMCTR1
        self.command(0xB2, &[0x05, 0x3A, 0x3A]); // FRMCTR2
        self.command(0xB3, &[0x05, 0x3A, 0x3A, 0x05, 0x3A, 0x3A]); // FRMCTR3
        self.command(0xB4, &[0x03]); // INVCTR
        self.command(0xC0, &[0x62, 0x02, 0x04]); // PWCTR1
        self.command(0xC1, &[0xC0]); // PWCTR2
        self.command(0xC2, &[0x0D, 0x00]); // PWCTR3
        self.command(0xC3, &[0x8D, 0x6A]); // PWCTR4
        self.command(0xC4, &[0x8D, 0xEE]); // PWCTR5
        self.command(0xC5, &[0x12]); // VMCTR1
        self.command(
            0xE0,
            &[
                0x03, 0x1B, 0x12, 0x11, 0x3F, 0x3A, 0x32, 0x34, 0x2F, 0x2B,
                0x30, 0x3A, 0x00, 0x00, 0x01, 0x05,
            ],
        ); // GMCTRP1
        self.command(
            0xE1,
            &[
                0x03, 0x1B, 0x12, 0x11, 0x32, 0x2F, 0x2A, 0x2F, 0x2E, 0x2C,
                0x35, 0x3F, 0x00, 0x00, 0x01, 0x05,
            ],
        ); // GMCTRN1
        self.command(0xFC, &[0x8C]); // vendor extension, purpose unknown
        self.command(0x36, &[0x60]); // MADCTL
        self.command(0x3A, &[0x05]); // COLMOD = RGB565
        delay::ms(syst, 10);
        self.command(0x29, &[]); // DISPON
    }
}

impl<'a> OriginDimensions for St7735<'a> {
    fn size(&self) -> Size {
        Size::new(WIDTH as u32, HEIGHT as u32)
    }
}

impl<'a> DrawTarget for St7735<'a> {
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn fill_contiguous<I>(
        &mut self,
        area: &Rectangle,
        colors: I,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let area = area.intersection(&self.bounding_box());
        if area.size.width == 0 || area.size.height == 0 {
            return Ok(());
        }
        let x0 = area.top_left.x as u16;
        let y0 = area.top_left.y as u16;
        let x1 = x0 + area.size.width as u16 - 1;
        let y1 = y0 + area.size.height as u16 - 1;
        self.set_window(x0, y0, x1, y1);
        self.write_command(0x2C); // RAMWR
        board::set_lcd_dc(self.gpiob, true);
        board::set_lcd_cs(self.gpiob, false);
        let n = area.size.width as usize * area.size.height as usize;
        for color in colors.into_iter().take(n) {
            self.stream_pixel(color);
        }
        self.spi_wait_idle();
        board::set_lcd_cs(self.gpiob, true);
        Ok(())
    }

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let bb = self.bounding_box();
        for Pixel(coord, color) in pixels {
            if !bb.contains(coord) {
                continue;
            }
            let x = coord.x as u16;
            let y = coord.y as u16;
            self.set_window(x, y, x, y);
            self.write_command(0x2C); // RAMWR
            board::set_lcd_dc(self.gpiob, true);
            board::set_lcd_cs(self.gpiob, false);
            self.stream_pixel(color);
            self.spi_wait_idle();
            board::set_lcd_cs(self.gpiob, true);
        }
        Ok(())
    }
}
