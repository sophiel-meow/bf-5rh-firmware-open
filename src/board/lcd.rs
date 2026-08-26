use at32f421_pac as pac;

const CS_BIT: u32 = 1 << 2; // PB2
const RST_BIT: u32 = 1 << 12; // PB12
const DC_BIT: u32 = 1 << 14; // PB14
const BACKLIGHT_BIT: u32 = 1 << 4; // PA4

/// PB2/PB12/PB14 push-pull output, idle high (CS/RESET deasserted). PB13/
/// PB15 = SPI2_SCK/MOSI, AF0.
pub fn init_lcd_control_pins(gpiob: &pac::Gpiob) {
    gpiob.cfgr().modify(|_, w| unsafe {
        w.iomc2()
            .bits(0b01)
            .iomc12()
            .bits(0b01)
            .iomc13()
            .bits(0b10)
            .iomc14()
            .bits(0b01)
            .iomc15()
            .bits(0b10)
    });
    gpiob
        .muxh()
        .modify(|_, w| unsafe { w.muxh13().bits(0).muxh15().bits(0) });
    gpiob.scr().write(|w| unsafe { w.bits(CS_BIT | RST_BIT) });
}

pub fn set_lcd_cs(gpiob: &pac::Gpiob, high: bool) {
    if high {
        gpiob.scr().write(|w| unsafe { w.bits(CS_BIT) });
    } else {
        gpiob.clr().write(|w| unsafe { w.bits(CS_BIT) });
    }
}

pub fn set_lcd_dc(gpiob: &pac::Gpiob, data_mode: bool) {
    if data_mode {
        gpiob.scr().write(|w| unsafe { w.bits(DC_BIT) });
    } else {
        gpiob.clr().write(|w| unsafe { w.bits(DC_BIT) });
    }
}

pub fn set_lcd_reset(gpiob: &pac::Gpiob, high: bool) {
    if high {
        gpiob.scr().write(|w| unsafe { w.bits(RST_BIT) });
    } else {
        gpiob.clr().write(|w| unsafe { w.bits(RST_BIT) });
    }
}

pub fn init_lcd_backlight_pin(gpioa: &pac::Gpioa) {
    gpioa.cfgr().modify(|_, w| unsafe { w.iomc4().bits(0b01) });
}

pub fn set_lcd_backlight(gpioa: &pac::Gpioa, on: bool) {
    if on {
        gpioa.scr().write(|w| unsafe { w.bits(BACKLIGHT_BIT) });
    } else {
        gpioa.clr().write(|w| unsafe { w.bits(BACKLIGHT_BIT) });
    }
}
