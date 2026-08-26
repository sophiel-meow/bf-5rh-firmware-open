#[doc = "Register `LOW_PULSE` reader"]
pub type R = crate::R<LowPulseSpec>;
#[doc = "Register `LOW_PULSE` writer"]
pub type W = crate::W<LowPulseSpec>;
#[doc = "Field `L_PULSE_CNT` reader - Low pulse Count"]
pub type LPulseCntR = crate::FieldReader;
#[doc = "Field `L_PULSE_CNT` writer - Low pulse Count"]
pub type LPulseCntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Low pulse Count"]
    #[inline(always)]
    pub fn l_pulse_cnt(&self) -> LPulseCntR {
        LPulseCntR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Low pulse Count"]
    #[inline(always)]
    pub fn l_pulse_cnt(&mut self) -> LPulseCntW<'_, LowPulseSpec> {
        LPulseCntW::new(self, 0)
    }
}
#[doc = "LOW_PULSE\n\nYou can [`read`](crate::Reg::read) this register and get [`low_pulse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`low_pulse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LowPulseSpec;
impl crate::RegisterSpec for LowPulseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`low_pulse::R`](R) reader structure"]
impl crate::Readable for LowPulseSpec {}
#[doc = "`write(|w| ..)` method takes [`low_pulse::W`](W) writer structure"]
impl crate::Writable for LowPulseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOW_PULSE to value 0"]
impl crate::Resettable for LowPulseSpec {}
