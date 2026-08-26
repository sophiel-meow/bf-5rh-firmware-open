use at32f421_pac as pac;

const TX_LED_BIT: u32 = 1 << 13; // PA13
const RX_LED_BIT: u32 = 1 << 14; // PA14

pub fn init_tx_rx_led_pins(gpioa: &pac::Gpioa) {
    gpioa
        .cfgr()
        .modify(|_, w| unsafe { w.iomc13().bits(0b01).iomc14().bits(0b01) });
}

pub fn set_tx_led(gpioa: &pac::Gpioa, on: bool) {
    if on {
        gpioa.scr().write(|w| unsafe { w.bits(TX_LED_BIT) });
    } else {
        gpioa.clr().write(|w| unsafe { w.bits(TX_LED_BIT) });
    }
}

pub fn set_rx_led(gpioa: &pac::Gpioa, on: bool) {
    if on {
        gpioa.scr().write(|w| unsafe { w.bits(RX_LED_BIT) });
    } else {
        gpioa.clr().write(|w| unsafe { w.bits(RX_LED_BIT) });
    }
}
