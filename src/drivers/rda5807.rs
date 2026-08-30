use crate::board;
use crate::hal::delay;
use at32f421_pac as pac;
use cortex_m::peripheral::SYST;

const BIT_DELAY_US: u32 = 10;

const ADDR_WRITE: u8 = 0x20;
const ADDR_READ: u8 = 0x21;

const CTRL_NORMAL: u16 = 0xD265;
const CTRL_SEEK: u16 = CTRL_NORMAL | 0x0100;
const CTRL_SEEK_DOWN: u16 = (CTRL_NORMAL & !0x0200) | 0x0100;
const CTRL_OFF: u16 = CTRL_NORMAL & !0x4001u16;

const CONFIG_REG04: u16 = 0x0400;
const CONFIG_REG05: u16 = 0x888F;

const BAND_WORLDWIDE: u16 = 0b10; // 76-108MHz
const BAND_LOW: u16 = 0b11; // 65-76MHz
const SPACE_100KHZ: u16 = 0b00;

const WORLDWIDE_BASE_KHZ: u32 = 76_000;
const LOW_BASE_KHZ: u32 = 65_000;
const CHANNEL_SPACING_KHZ: u32 = 100;

const STATUS_STC: u16 = 0x4000;
const STATUS_SF: u16 = 0x2000;
const STATUS_READCHAN_MASK: u16 = 0x03FF;
const STATUS_FM_TRUE: u16 = 0x0100;

#[allow(dead_code)]
pub const FREQ_LO_KHZ: u32 = LOW_BASE_KHZ;
#[allow(dead_code)]
pub const FREQ_HI_KHZ: u32 = 108_000;

pub struct Rda5807<'a> {
    gpioa: &'a pac::Gpioa,
    gpioc: &'a pac::Gpioc,
    last_band_base_khz: u32,
}

impl<'a> Rda5807<'a> {
    pub fn new(gpioa: &'a pac::Gpioa, gpioc: &'a pac::Gpioc) -> Self {
        Rda5807 {
            gpioa,
            gpioc,
            last_band_base_khz: WORLDWIDE_BASE_KHZ,
        }
    }

    fn start(&mut self, syst: &mut SYST) {
        board::set_shared_sda(self.gpioa, true);
        delay::us(syst, BIT_DELAY_US);
        board::set_rda5807_scl(self.gpioc, true);
        delay::us(syst, BIT_DELAY_US);
        board::set_shared_sda(self.gpioa, false);
        delay::us(syst, BIT_DELAY_US);
        board::set_rda5807_scl(self.gpioc, false);
        delay::us(syst, BIT_DELAY_US);
    }

    fn stop(&mut self, syst: &mut SYST) {
        board::set_rda5807_scl(self.gpioc, false);
        delay::us(syst, BIT_DELAY_US);
        board::set_shared_sda(self.gpioa, false);
        delay::us(syst, BIT_DELAY_US);
        board::set_rda5807_scl(self.gpioc, true);
        delay::us(syst, BIT_DELAY_US);
        board::set_shared_sda(self.gpioa, true);
        delay::us(syst, BIT_DELAY_US);
    }

    fn write_byte(&mut self, syst: &mut SYST, mut byte: u8) -> bool {
        for _ in 0..8 {
            board::set_shared_sda(self.gpioa, byte & 0x80 != 0);
            byte <<= 1;
            delay::us(syst, BIT_DELAY_US);
            board::set_rda5807_scl(self.gpioc, true);
            delay::us(syst, BIT_DELAY_US);
            board::set_rda5807_scl(self.gpioc, false);
            delay::us(syst, BIT_DELAY_US);
        }

        board::set_shared_sda_input(self.gpioa);
        delay::us(syst, BIT_DELAY_US);
        board::set_rda5807_scl(self.gpioc, true);
        delay::us(syst, BIT_DELAY_US);
        let ack = !board::read_shared_sda(self.gpioa);
        board::set_rda5807_scl(self.gpioc, false);
        delay::us(syst, BIT_DELAY_US);
        board::set_shared_sda_output(self.gpioa);
        ack
    }

    fn read_byte(&mut self, syst: &mut SYST, ack: bool) -> u8 {
        let mut data = 0u8;
        board::set_shared_sda_input(self.gpioa);
        delay::us(syst, BIT_DELAY_US);
        for _ in 0..8 {
            board::set_rda5807_scl(self.gpioc, true);
            delay::us(syst, BIT_DELAY_US);
            data <<= 1;
            if board::read_shared_sda(self.gpioa) {
                data |= 1;
            }
            board::set_rda5807_scl(self.gpioc, false);
            delay::us(syst, BIT_DELAY_US);
        }

        board::set_shared_sda_output(self.gpioa);
        board::set_shared_sda(self.gpioa, !ack);
        delay::us(syst, BIT_DELAY_US);
        board::set_rda5807_scl(self.gpioc, true);
        delay::us(syst, BIT_DELAY_US);
        board::set_rda5807_scl(self.gpioc, false);
        delay::us(syst, BIT_DELAY_US);
        data
    }

    fn write_regs(&mut self, syst: &mut SYST, regs: &[u16]) {
        self.start(syst);
        self.write_byte(syst, ADDR_WRITE);
        for &value in regs {
            self.write_byte(syst, (value >> 8) as u8);
            self.write_byte(syst, value as u8);
        }
        self.stop(syst);
    }

    fn read_status(&mut self, syst: &mut SYST) -> (u16, u16) {
        self.start(syst);
        self.write_byte(syst, ADDR_READ);
        let a_hi = self.read_byte(syst, true);
        let a_lo = self.read_byte(syst, true);
        let b_hi = self.read_byte(syst, true);
        let b_lo = self.read_byte(syst, false);
        self.stop(syst);
        (
            ((a_hi as u16) << 8) | a_lo as u16,
            ((b_hi as u16) << 8) | b_lo as u16,
        )
    }

    pub fn set_frequency_khz(&mut self, syst: &mut SYST, freq_khz: u32) {
        let (base, band) = if freq_khz >= WORLDWIDE_BASE_KHZ {
            (WORLDWIDE_BASE_KHZ, BAND_WORLDWIDE)
        } else {
            (LOW_BASE_KHZ, BAND_LOW)
        };
        self.last_band_base_khz = base;
        let chan = freq_khz.saturating_sub(base) / CHANNEL_SPACING_KHZ;
        let reg03 =
            ((chan as u16) << 6) | (1 << 4) | (band << 2) | SPACE_100KHZ;
        self.write_regs(
            syst,
            &[CTRL_NORMAL, reg03, CONFIG_REG04, CONFIG_REG05],
        );
    }

    #[allow(dead_code)]
    pub fn seek(&mut self, syst: &mut SYST, up: bool) {
        self.write_regs(syst, &[if up { CTRL_SEEK } else { CTRL_SEEK_DOWN }]);
    }

    #[allow(dead_code)]
    pub fn tuned_frequency_khz(&mut self, syst: &mut SYST) -> u32 {
        let (a, _b) = self.read_status(syst);
        let chan = (a & STATUS_READCHAN_MASK) as u32;
        self.last_band_base_khz + chan * CHANNEL_SPACING_KHZ
    }

    #[allow(dead_code)]
    pub fn power_off(&mut self, syst: &mut SYST) {
        self.write_regs(syst, &[CTRL_OFF]);
    }

    pub fn status(&mut self, syst: &mut SYST) -> (bool, bool, bool, u8) {
        let (a, b) = self.read_status(syst);
        let complete = a & STATUS_STC != 0;
        let seek_failed = a & STATUS_SF != 0;
        let is_station = b & STATUS_FM_TRUE != 0;
        let rssi = (b >> 9) as u8;
        (complete, seek_failed, is_station, rssi)
    }
}
