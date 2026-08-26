use at32f421_pac as pac;

pub const SCLK_HZ: u32 = 120_000_000;

pub fn configure_system_clock(crm: &pac::Crm, flash: &pac::Flash) {
    if crm.cfg().read().sclksts().bits() != 0b00 {
        crm.cfg().modify(|_, w| unsafe { w.sclksel().bits(0b00) });
        while crm.cfg().read().sclksts().bits() != 0b00 {}
    }
    if crm.ctrl().read().pllen().bit_is_set() {
        crm.ctrl().modify(|_, w| w.pllen().clear_bit());
        while crm.ctrl().read().pllstbl().bit_is_set() {}
    }

    crm.ctrl().modify(|_, w| w.hicken().set_bit());
    while crm.ctrl().read().hickstbl().bit_is_clear() {}

    crm.ctrl().modify(|_, w| w.hexten().set_bit());
    while crm.ctrl().read().hextstbl().bit_is_clear() {}

    flash.psr().write(|w| unsafe { w.bits(0x153) });

    // PLLRCS=1 (HEXT) -> PLLHEXTDIV=1 (HEXT/2 = 8MHz PLL entry
    // reference) -> PLLMULT = 00_1101 ->
    // 8MHz * 15 = 120MHz
    crm.cfg().modify(|_, w| unsafe {
        w.pllrcs()
            .set_bit()
            .pllhextdiv()
            .set_bit()
            .pllmult3_0()
            .bits(0b1101)
            .pllmult5_4()
            .bits(0b00)
    });

    crm.ctrl().modify(|_, w| w.pllen().set_bit());
    while crm.ctrl().read().pllstbl().bit_is_clear() {}

    crm.cfg().modify(|_, w| unsafe { w.sclksel().bits(0b10) });
    while crm.cfg().read().sclksts().bits() != 0b10 {}
}
