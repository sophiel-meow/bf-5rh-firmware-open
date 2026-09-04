use at32f421_pac as pac;

const COL_MASK: u32 = 0x0F00; // PB8-11
const ROW_MASK: u32 = 0x0078; // PB3-6
const SIDE2_BIT: u32 = 1 << 15; // PA15

/// PB8-11 push-pull output (idle high = no column selected). PB3-6 input
/// with pull-up.
pub fn init_keypad_pins(gpiob: &pac::Gpiob) {
    gpiob.cfgr().modify(|_, w| unsafe {
        w.iomc8()
            .bits(0b01)
            .iomc9()
            .bits(0b01)
            .iomc10()
            .bits(0b01)
            .iomc11()
            .bits(0b01)
            .iomc3()
            .bits(0b00)
            .iomc4()
            .bits(0b00)
            .iomc5()
            .bits(0b00)
            .iomc6()
            .bits(0b00)
    });
    gpiob.pull().modify(|_, w| unsafe {
        w.pull3()
            .bits(0b01)
            .pull4()
            .bits(0b01)
            .pull5()
            .bits(0b01)
            .pull6()
            .bits(0b01)
    });
    set_keypad_rows_idle(gpiob);
}

pub fn set_keypad_rows_idle(gpiob: &pac::Gpiob) {
    gpiob.scr().write(|w| unsafe { w.bits(COL_MASK) });
}

pub fn set_keypad_row(gpiob: &pac::Gpiob, phase: u8) {
    set_keypad_rows_idle(gpiob);
    let pin = match phase {
        1 => 8u8,
        2 => 9,
        3 => 10,
        4 => 11,
        _ => return,
    };
    gpiob.clr().write(|w| unsafe { w.bits(1u32 << pin) });
}

pub fn read_keypad_column(gpiob: &pac::Gpiob) -> Option<u8> {
    let rows_low = !gpiob.idt().read().bits() & ROW_MASK;
    if rows_low == 0 {
        None
    } else {
        Some((rows_low >> 3).trailing_zeros() as u8)
    }
}

pub fn init_side_key2_pin(gpioa: &pac::Gpioa) {
    gpioa.cfgr().modify(|_, w| unsafe { w.iomc15().bits(0b00) });
    gpioa.pull().modify(|_, w| unsafe { w.pull15().bits(0b01) });
}

pub fn read_side_key2(gpioa: &pac::Gpioa) -> bool {
    gpioa.idt().read().bits() & SIDE2_BIT == 0
}
