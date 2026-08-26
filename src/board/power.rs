use at32f421_pac as pac;

const POWER_LATCH_BIT: u32 = 1 << 6; // PF6

pub fn init_power_latch_pin(gpiof: &pac::Gpiof) {
    gpiof.cfgr().modify(|_, w| unsafe { w.iomc6().bits(0b01) });
}

pub fn set_power_latch(gpiof: &pac::Gpiof, on: bool) {
    if on {
        gpiof.scr().write(|w| unsafe { w.bits(POWER_LATCH_BIT) });
    } else {
        gpiof.clr().write(|w| unsafe { w.bits(POWER_LATCH_BIT) });
    }
}

const POWER_KEY_BIT: u32 = 1 << 0; // PB0

pub fn init_power_key_pin(gpiob: &pac::Gpiob) {
    gpiob.cfgr().modify(|_, w| unsafe { w.iomc0().bits(0b00) });
    gpiob.pull().modify(|_, w| unsafe { w.pull0().bits(0b01) });
}

pub fn read_power_key(gpiob: &pac::Gpiob) -> bool {
    gpiob.idt().read().bits() & POWER_KEY_BIT != 0
}
