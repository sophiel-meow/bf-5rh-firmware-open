const PATTERN: u32 = 0xA5A5_A5A5;

const PAINT_MARGIN: u32 = 128;

unsafe extern "C" {
    static __sheap: u32;
    static _stack_start: u32;
}

fn heap_start() -> u32 {
    &raw const __sheap as u32
}

fn stack_top() -> u32 {
    &raw const _stack_start as u32
}

#[cortex_m_rt::pre_init]
unsafe fn paint_stack() {
    let sp: u32;
    unsafe {
        core::arch::asm!("mov {}, sp", out(reg) sp, options(nomem, nostack, preserves_flags));
    }

    let lo = heap_start();
    let hi = sp.saturating_sub(PAINT_MARGIN);

    let mut p = lo as *mut u32;
    while (p as u32) < hi {
        unsafe {
            p.write_volatile(PATTERN);
            p = p.add(1);
        }
    }
}

#[allow(dead_code)]
pub fn high_water() -> (u32, u32) {
    let lo = heap_start();
    let top = stack_top();

    let mut p = lo as *const u32;
    while (p as u32) < top {
        if unsafe { p.read_volatile() } != PATTERN {
            break;
        }
        p = unsafe { p.add(1) };
    }

    let low_mark = p as u32;
    (top - low_mark, low_mark - lo)
}
