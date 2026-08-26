use crate::drivers::rda5807::Rda5807;
use at32f421_pac as pac;
use cortex_m::peripheral::SYST;

pub use crate::drivers::rda5807::{FREQ_HI_KHZ, FREQ_LO_KHZ};

pub struct FmRadio<'a> {
    chip: Rda5807<'a>,
}

impl<'a> FmRadio<'a> {
    pub fn new(gpioa: &'a pac::Gpioa, gpioc: &'a pac::Gpioc) -> Self {
        FmRadio {
            chip: Rda5807::new(gpioa, gpioc),
        }
    }

    pub fn tune_khz(&mut self, syst: &mut SYST, freq_khz: u32) {
        self.chip.set_frequency_khz(syst, freq_khz);
    }

    pub fn seek(&mut self, syst: &mut SYST, up: bool) {
        self.chip.seek(syst, up);
    }

    pub fn status(&mut self, syst: &mut SYST) -> (bool, bool, bool, u8) {
        self.chip.status(syst)
    }

    pub fn tuned_frequency_khz(&mut self, syst: &mut SYST) -> u32 {
        self.chip.tuned_frequency_khz(syst)
    }

    pub fn power_off(&mut self, syst: &mut SYST) {
        self.chip.power_off(syst);
    }
}
