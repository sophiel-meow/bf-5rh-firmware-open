use at32f421_pac as pac;

const NORFLASH_CS_BIT: u32 = 1 << 8; // PA8

/// SCK=PA5, MISO=PA6, MOSI=PA7, AF mux 0
pub fn init_norflash_pins(gpioa: &pac::Gpioa) {
    gpioa.cfgr().modify(|_, w| unsafe {
        w.iomc5()
            .bits(0b10)
            .iomc6()
            .bits(0b10)
            .iomc7()
            .bits(0b10)
            .iomc8()
            .bits(0b01)
    });
    gpioa.muxl().modify(|_, w| unsafe {
        w.muxl5().bits(0).muxl6().bits(0).muxl7().bits(0)
    });
    gpioa.pull().modify(|_, w| unsafe {
        w.pull5().bits(0b00).pull6().bits(0b00).pull7().bits(0b00)
    });
    gpioa.scr().write(|w| unsafe { w.bits(NORFLASH_CS_BIT) }); // CS idle high
}

pub fn set_norflash_cs(gpioa: &pac::Gpioa, high: bool) {
    if high {
        gpioa.scr().write(|w| unsafe { w.bits(NORFLASH_CS_BIT) });
    } else {
        gpioa.clr().write(|w| unsafe { w.bits(NORFLASH_CS_BIT) });
    }
}
