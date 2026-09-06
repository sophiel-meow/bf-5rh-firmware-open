use at32f421_pac as pac;

use crate::hal::clock::SCLK_HZ;

const TICK_HZ: u32 = 10_000;
const DIV: u16 = ((SCLK_HZ + TICK_HZ / 2) / TICK_HZ - 1) as u16;

pub fn init() {
    let tmr = unsafe { &*pac::Tmr14::ptr() };
    tmr.ctrl1().write(|w| w.tmren().clear_bit());
    tmr.div().write(|w| unsafe { w.div().bits(DIV) });
    tmr.pr().write(|w| unsafe { w.pr().bits(0xFFFF) });
    tmr.swevt().write(|w| w.ovfswtr().set_bit());
    tmr.ctrl1().write(|w| w.tmren().set_bit());
}

/// Current tick count, in units of 100us. Wraps every 6.5536s.
pub fn now() -> u16 {
    let tmr = unsafe { &*pac::Tmr14::ptr() };
    tmr.cval().read().cval().bits()
}

static mut LAST16: u16 = 0;
static mut HIGH: u32 = 0;

pub fn now32() -> u32 {
    let n = now();
    unsafe {
        if n < LAST16 {
            HIGH = HIGH.wrapping_add(0x1_0000);
        }
        LAST16 = n;
        HIGH.wrapping_add(n as u32)
    }
}
