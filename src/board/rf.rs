use at32f421_pac as pac;

const LE_BIT: u32 = 1 << 13; // PC13
const SCK_BIT: u32 = 1 << 2; // PA2
const SDA_BIT: u32 = 1 << 3; // PA3

/// PA2/PA3 push-pull output, PC13 push-pull output (LE idle high = no
/// transaction in progress).
pub fn init_fd6818_pins(gpioa: &pac::Gpioa, gpioc: &pac::Gpioc) {
    gpioa
        .cfgr()
        .modify(|_, w| unsafe { w.iomc2().bits(0b01).iomc3().bits(0b01) });
    gpioa
        .pull()
        .modify(|_, w| unsafe { w.pull2().bits(0b00).pull3().bits(0b00) });
    gpioc.cfgr().modify(|_, w| unsafe { w.iomc13().bits(0b01) });
    gpioc.scr().write(|w| unsafe { w.bits(LE_BIT) });
}

pub fn set_fd6818_scn(gpioc: &pac::Gpioc, high: bool) {
    if high {
        gpioc.scr().write(|w| unsafe { w.bits(LE_BIT) });
    } else {
        gpioc.clr().write(|w| unsafe { w.bits(LE_BIT) });
    }
}

pub fn set_fd6818_sck(gpioa: &pac::Gpioa, high: bool) {
    if high {
        gpioa.scr().write(|w| unsafe { w.bits(SCK_BIT) });
    } else {
        gpioa.clr().write(|w| unsafe { w.bits(SCK_BIT) });
    }
}

/// PA3 is physically shared between the FD6818/BK4829 3-wire bus's SDIO
/// line and the RDA5807's bit-banged I2C SDA line
pub fn set_shared_sda(gpioa: &pac::Gpioa, high: bool) {
    if high {
        gpioa.scr().write(|w| unsafe { w.bits(SDA_BIT) });
    } else {
        gpioa.clr().write(|w| unsafe { w.bits(SDA_BIT) });
    }
}

pub fn set_shared_sda_input(gpioa: &pac::Gpioa) {
    gpioa.cfgr().modify(|_, w| unsafe { w.iomc3().bits(0b00) });
    gpioa.pull().modify(|_, w| unsafe { w.pull3().bits(0b01) });
}

pub fn set_shared_sda_output(gpioa: &pac::Gpioa) {
    gpioa.cfgr().modify(|_, w| unsafe { w.iomc3().bits(0b01) });
    gpioa.pull().modify(|_, w| unsafe { w.pull3().bits(0b00) });
}

pub fn read_shared_sda(gpioa: &pac::Gpioa) -> bool {
    gpioa.idt().read().bits() & SDA_BIT != 0
}

/// Audio amplifier enable/mute: PB7, active high
pub fn init_speaker_switch_pin(gpiob: &pac::Gpiob) {
    gpiob.cfgr().modify(|_, w| unsafe { w.iomc7().bits(0b01) });
}

pub fn set_speaker_switch(gpiob: &pac::Gpiob, on: bool) {
    const AMP_BIT: u32 = 1 << 7;
    if on {
        gpiob.scr().write(|w| unsafe { w.bits(AMP_BIT) });
    } else {
        gpiob.clr().write(|w| unsafe { w.bits(AMP_BIT) });
    }
}
