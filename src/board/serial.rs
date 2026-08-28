use at32f421_pac as pac;

const PTT_BIT: u32 = 1 << 10; // PA10

pub fn init_debug_uart_tx_pin(gpioa: &pac::Gpioa) {
    gpioa.cfgr().modify(|_, w| unsafe { w.iomc9().bits(0b10) });
    gpioa.muxh().modify(|_, w| unsafe { w.muxh9().bits(1) });
}

pub fn init_ptt_rxd_pin(gpioa: &pac::Gpioa) {
    gpioa.cfgr().modify(|_, w| unsafe { w.iomc10().bits(0b00) });
    gpioa.pull().modify(|_, w| unsafe { w.pull10().bits(0b01) });
}

pub fn init_usart1_rx_pin(gpioa: &pac::Gpioa) {
    gpioa.cfgr().modify(|_, w| unsafe { w.iomc10().bits(0b10) });
    gpioa.muxh().modify(|_, w| unsafe { w.muxh10().bits(1) });
}

/// `true` while PTT is pressed (active low).
pub fn read_ptt(gpioa: &pac::Gpioa) -> bool {
    gpioa.idt().read().bits() & PTT_BIT == 0
}
