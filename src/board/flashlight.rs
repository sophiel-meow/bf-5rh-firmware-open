use at32f421_pac as pac;

const FLASHLIGHT_BIT: u32 = 1 << 7; // PF7

pub fn init_flashlight_pin(gpiof: &pac::Gpiof) {
    gpiof.cfgr().modify(|_, w| unsafe { w.iomc7().bits(0b01) });
}

pub fn set_flashlight(gpiof: &pac::Gpiof, on: bool) {
    if on {
        gpiof.scr().write(|w| unsafe { w.bits(FLASHLIGHT_BIT) });
    } else {
        gpiof.clr().write(|w| unsafe { w.bits(FLASHLIGHT_BIT) });
    }
}
