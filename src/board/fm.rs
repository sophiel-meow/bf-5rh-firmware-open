use at32f421_pac as pac;

const RDA_SCL_BIT: u32 = 1 << 15; // PC15

pub fn init_rda5807_scl_pin(gpioc: &pac::Gpioc) {
    gpioc.cfgr().modify(|_, w| unsafe { w.iomc15().bits(0b01) });
    gpioc.scr().write(|w| unsafe { w.bits(RDA_SCL_BIT) });
}

pub fn set_rda5807_scl(gpioc: &pac::Gpioc, high: bool) {
    if high {
        gpioc.scr().write(|w| unsafe { w.bits(RDA_SCL_BIT) });
    } else {
        gpioc.clr().write(|w| unsafe { w.bits(RDA_SCL_BIT) });
    }
}
