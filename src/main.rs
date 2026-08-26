#![no_std]
#![no_main]

mod app;
mod board;
mod device;
mod drivers;
mod flash_map;
mod hal;
mod ui;

use at32f421_pac as pac;
use core::fmt::Write;
use cortex_m_rt::entry;
use panic_halt as _;

use device::radio::{AniConfig, ChannelConfig, Modulation, Power, Radio, SubAudio};
use drivers::fd6818b::Fd6818;
use drivers::norflash::NorFlash;
use hal::clock::{self, SCLK_HZ};

#[cfg_attr(not(debug_assertions), allow(dead_code))]
const BAUD: u32 = 115_200;
const CYCLES_PER_MS: u32 = SCLK_HZ / 1_000;

const TEST_FREQ_HZ: u32 = 439_500_000;
const WIDE_BAND: bool = true;

fn delay_ms(ms: u32) {
    cortex_m::asm::delay(ms.saturating_mul(CYCLES_PER_MS));
}

/// Serial diagnostics only ever run in a debug (`cargo build`, no
/// `--release`) image -- this board's 60K flash budget (`memory.x`) has no
/// room to spare for `core::fmt` formatting machinery in the shipped
/// release binary. Every call site uses this instead of `writeln!` directly
/// so the argument expressions (and any `{..}` formatting they pull in)
/// vanish entirely under `--release` rather than merely going silent.
macro_rules! dbg_println {
    ($serial:expr, $($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            writeln!($serial, $($arg)*).ok();
        }
        #[cfg(not(debug_assertions))]
        {
            let _ = &$serial;
        }
    }};
}

struct Serial {
    usart1: pac::Usart1,
}

impl Serial {
    fn write_byte(&mut self, b: u8) {
        while self.usart1.sts().read().tdbe().bit_is_clear() {}
        self.usart1.dt().write(|w| unsafe { w.dt().bits(b as u16) });
    }
}

impl Write for Serial {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for &b in s.as_bytes() {
            if b == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(b);
        }
        Ok(())
    }
}

impl Serial {
    #[cfg_attr(not(debug_assertions), allow(dead_code))]
    fn flush(&mut self) {
        while self.usart1.sts().read().tdc().bit_is_clear() {}
    }
}

fn check_power_off(
    gpiob: &pac::Gpiob,
    gpiof: &pac::Gpiof,
    serial: &mut Serial,
    app: &mut app::App<'_>,
    syst: &mut cortex_m::peripheral::SYST,
) {
    if !board::read_power_key(gpiob) {
        return;
    }
    delay_ms(50);
    if !board::read_power_key(gpiob) {
        return;
    }
    dbg_println!(serial, "power key: shutdown detected, dropping PF6 latch");
    #[cfg(debug_assertions)]
    serial.flush();
    app.radio_mut().rf_off(syst);
    app.save_channel_state();
    board::set_power_latch(gpiof, false);
    loop {
        cortex_m::asm::wfi();
    }
}

#[entry]
fn main() -> ! {
    let dp = unsafe { pac::Peripherals::steal() };
    let mut cp = cortex_m::Peripherals::take().unwrap();

    // GPIOF clock for PF6 (power self-latch)
    dp.crm.ahben().modify(|_, w| w.gpiofen().set_bit());
    let _ = dp.crm.ahben().read();

    // PF6 = power self-latch, driven high
    board::init_power_latch_pin(&dp.gpiof);
    board::set_power_latch(&dp.gpiof, true);

    clock::configure_system_clock(&dp.crm, &dp.flash);

    // Remaining clocks: GPIOA/B/C + USART1 + SPI1 (calibration flash)
    dp.crm.ahben().modify(|_, w| {
        w.gpioaen()
            .set_bit()
            .gpioben()
            .set_bit()
            .gpiocen()
            .set_bit()
    });
    dp.crm
        .apb2en()
        .modify(|_, w| w.usart1en().set_bit().spi1en().set_bit().adcen().set_bit());
    dp.crm
        .apb1en()
        .modify(|_, w| w.spi2en().set_bit().tmr14en().set_bit());
    let _ = dp.crm.ahben().read();

    // PA0/PA1 analog mode for the VOX mic level / battery voltage ADC
    dp.gpioa
        .cfgr()
        .modify(|_, w| unsafe { w.iomc0().bits(0b11).iomc1().bits(0b11) });
    dp.gpioa
        .pull()
        .modify(|_, w| unsafe { w.pull0().bits(0b00).pull1().bits(0b00) });

    // TMR14 free-running 100us tick, backing `device::keypad::Keypad`'s
    // debounce/long-press/repeat timing (`hal/uptime.rs`).
    hal::uptime::init();

    // PB0 = power key / detect input
    board::init_power_key_pin(&dp.gpiob);

    // PA9 = USART1_TX (AF mux 1).
    board::init_debug_uart_tx_pin(&dp.gpioa);

    // PA10 = PTT, input with pull-up.
    board::init_ptt_rxd_pin(&dp.gpioa);

    // PA13/PA14 = TX/RX indicator LEDs
    board::init_tx_rx_led_pins(&dp.gpioa);
    board::set_tx_led(&dp.gpioa, false);
    board::set_rx_led(&dp.gpioa, false);

    // PB7 = audio amp enable, push-pull output, driven high (amp on) to start.
    board::init_speaker_switch_pin(&dp.gpiob);
    board::set_speaker_switch(&dp.gpiob, true);

    // RFIC bus pins.
    board::init_fd6818_pins(&dp.gpioa, &dp.gpioc);

    // Calibration flash: SPI1 master, mode 0, software CS, 8-bit frames,
    // ~3.4 MHz (SCLK_HZ / 32) -- plenty slow/safe for a JEDEC SPI-NOR read.
    board::init_norflash_pins(&dp.gpioa);
    dp.spi1.ctrl2().write(|w| w.mdiv3().clear_bit());
    dp.spi1.ctrl1().write(|w| unsafe {
        w.msten()
            .set_bit()
            .clkpha()
            .clear_bit()
            .clkpol()
            .clear_bit()
            .mdiv2_0()
            .bits(0b100)
            .ltf()
            .clear_bit()
            .swcsen()
            .set_bit()
            .swcsil()
            .set_bit()
            .fbn()
            .clear_bit()
            .spien()
            .set_bit()
    });

    board::init_lcd_control_pins(&dp.gpiob);
    board::init_lcd_backlight_pin(&dp.gpioa);
    dp.spi2.ctrl1().write(|w| unsafe {
        w.msten()
            .set_bit()
            .mdiv2_0()
            .bits(2)
            .swcsen()
            .set_bit()
            .swcsil()
            .set_bit()
    });
    dp.spi2.ctrl1().modify(|_, w| w.spien().set_bit());

    // Keypad matrix
    board::init_keypad_pins(&dp.gpiob);
    board::init_side_key2_pin(&dp.gpioa);

    // uart
    let div: u16 = 1042;
    dp.usart1.baudr().write(|w| unsafe { w.div().bits(div) });
    dp.usart1.ctrl1().write(|w| {
        w.uen()
            .set_bit()
            .rdbfien()
            .set_bit()
            .ten()
            .set_bit()
            .ren()
            .set_bit()
    });

    let mut serial = Serial { usart1: dp.usart1 };
    dbg_println!(serial, "\r\nbf5rh-fw: FD6818B/BK4829");
    dbg_println!(
        serial,
        "sclk={SCLK_HZ}Hz (HEXT+PLL, 16MHz crystal) baud={BAUD} div={div}"
    );

    // LCD init only -- the old Phase-1 "Hello, World!" smoke-test draw
    // (fill_screen + a test Text) lived here before the app/ui layer existed,
    // but `ui::draw` repaints the whole screen from `app` state on the very
    // first tick, so that one-shot test draw was pure flash-budget cost (this
    // board's 60K usable flash has none to spare, see `memory.x`).
    let mut display = device::display::Display::new(&dp.gpiob, &dp.spi2);
    display.init(&mut cp.SYST);
    let backlight = device::display::Backlight::new(&dp.gpioa);
    backlight.on();
    dbg_println!(serial, "lcd: init done");

    let mut rfic = Fd6818::new(&dp.gpioa, &dp.gpioc);
    let chip_id = rfic.chip_id(&mut cp.SYST);
    dbg_println!(serial, "fd6818 chip id = {chip_id:#06x} (expect 0x4829)");

    rfic.init(&mut cp.SYST);
    dbg_println!(serial, "fd6818 init done");

    let norflash = NorFlash::new(&dp.gpioa, &dp.spi1);
    let mut storage = device::storage::Storage::new(norflash);
    let mut flag_byte = [0u8; 1];
    storage.read_raw(flash_map::addr::PA_TABLE_BASE_HIGH, &mut flag_byte);
    let cal = storage.read_calibration();
    if flag_byte[0] != 0xFF {
        rfic.set_xtal_adjust(cal[6]);
        rfic.set_audio_calibration(cal[0], cal[1], cal[2], cal[3], cal[4]);
    }
    dbg_println!(
        serial,
        "calibration {}",
        if flag_byte[0] != 0xFF {
            "applied"
        } else {
            "blank, using defaults"
        }
    );

    // AF response filter coefficients (REG 0x44/45/54/55/74/75): the 5RH's
    // calibration block has no AF-response entries, so this pushes the
    // driver's own default/"flat" table entry (db=0) instead of leaving
    // these at chip power-on-reset state.
    rfic.apply_af_calibration(&mut cp.SYST, 0, 0, 0, 0);

    let cfg = ChannelConfig {
        freq_hz: TEST_FREQ_HZ,
        tx_freq_hz: TEST_FREQ_HZ,
        wide_band: WIDE_BAND,
        power: Power::Low,
        subaudio_tx: SubAudio::None,
        subaudio_rx: SubAudio::None,
        modulation: Modulation::Fm,
    };

    // FIXME: placeholder
    let ani = AniConfig::from_raw([0, 0, 0], 0);
    let radio = Radio::new(rfic, &dp.gpioa, &dp.gpiob, &dp.adc, cfg, ani);

    board::init_rda5807_scl_pin(&dp.gpioc);
    let fm_radio = device::fm_radio::FmRadio::new(&dp.gpioa, &dp.gpioc);

    let flashlight = device::flashlight::Flashlight::new(&dp.gpiof);
    let keypad = device::keypad::Keypad::new(&dp.gpioa, &dp.gpiob);
    dbg_println!(serial, "app: starting");

    let mut app = app::App::new(
        radio,
        chip_id,
        keypad,
        storage,
        flashlight,
        fm_radio,
        cfg,
        &mut cp.SYST,
    );
    app.init_battery_samples();
    let mut ui_state = ui::UiState::new();

    let mut fast_tick: u32 = 0;
    let mut real_tick10_last = hal::uptime::now();
    loop {
        check_power_off(&dp.gpiob, &dp.gpiof, &mut serial, &mut app, &mut cp.SYST);
        app.poll_keys(&mut cp.SYST);

        if let Some(level) = app.radio_mut().poll_ptt() {
            app.set_ptt(&mut cp.SYST, level);
        }

        let elapsed10 = hal::uptime::now().wrapping_sub(real_tick10_last);
        if elapsed10 >= 100 {
            let n10 = elapsed10 / 100;
            real_tick10_last = real_tick10_last.wrapping_add(n10.wrapping_mul(100));
            app.poll_tot(&mut cp.SYST, n10);
            app.poll_no_channels_notice(n10);
        }

        if fast_tick % 5 == 0 {
            let rx_active = app.rssi_open() || app.audio_open();
            if !app.is_transmitting() {
                app.poll_squelch(&mut cp.SYST, 2);
            }
            app.poll_dual_standby(&mut cp.SYST, rx_active);
            app.poll_power_save(&mut cp.SYST);
            app.poll_auto_lock(&mut cp.SYST, rx_active);
            app.poll_backlight();
            app.poll_blink();
            app.poll_chanmgr_name_timeout();
            app.poll_scan(&mut cp.SYST);
            app.poll_search(&mut cp.SYST);
            app.poll_scanqt(&mut cp.SYST);
            app.poll_fm(&mut cp.SYST);

            if !app.power_save_active() {
                let mic = app.radio_mut().read_mic_level(&mut cp.SYST);
                app.set_mic_level(mic);
                app.poll_vox(&mut cp.SYST, mic, rx_active);
            }
            app.poll_battery();

            board::set_rx_led(&dp.gpioa, rx_active && !app.is_transmitting());
            board::set_tx_led(&dp.gpioa, app.is_transmitting());
            backlight_set(&dp.gpioa, app.backlight_should_be_on());
        }

        if fast_tick % 15 == 0 {
            if !app.is_transmitting() && !app.power_save_is_asleep() {
                let rssi = app.radio_mut().rssi(&mut cp.SYST);
                app.set_rssi_raw(rssi as u8);
            }
            ui::draw(&mut display, &mut app, &mut ui_state);
        }

        fast_tick = fast_tick.wrapping_add(1);
        delay_ms(2);
    }
}

fn backlight_set(gpioa: &pac::Gpioa, on: bool) {
    board::set_lcd_backlight(gpioa, on);
}
