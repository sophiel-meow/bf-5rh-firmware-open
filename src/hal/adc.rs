use at32f421_pac as pac;

const SAMPLE_TIME_SLOW: u8 = 0b110;

pub struct Adc<'a> {
    adc: &'a pac::Adc,
}

impl<'a> Adc<'a> {
    pub fn new(adc: &'a pac::Adc) -> Self {
        adc.ctrl2().modify(|_, w| w.adcen().set_bit());

        adc.ctrl2().modify(|_, w| w.adcalinit().set_bit());
        while adc.ctrl2().read().adcalinit().bit_is_set() {}
        adc.ctrl2().modify(|_, w| w.adcal().set_bit());
        while adc.ctrl2().read().adcal().bit_is_set() {}

        adc.ctrl2()
            .modify(|_, w| unsafe { w.octesel().bits(0b111).octen().set_bit() });

        Adc { adc }
    }

    pub fn read_channel(&mut self, channel: u8) -> u16 {
        match channel {
            0..=9 => {
                let shift = (channel as u32) * 3;
                self.adc.spt2().modify(|r, w| unsafe {
                    w.bits((r.bits() & !(0b111 << shift)) | ((SAMPLE_TIME_SLOW as u32) << shift))
                });
            }
            _ => {
                let shift = ((channel as u32) - 10) * 3;
                self.adc.spt1().modify(|r, w| unsafe {
                    w.bits((r.bits() & !(0b111 << shift)) | ((SAMPLE_TIME_SLOW as u32) << shift))
                });
            }
        }

        self.adc
            .osq3()
            .modify(|_, w| unsafe { w.osn1().bits(channel) });
        self.adc.ctrl2().modify(|_, w| w.ocswtrg().set_bit());
        while self.adc.sts().read().cce().bit_is_clear() {}
        self.adc.odt().read().odt().bits()
    }
}
