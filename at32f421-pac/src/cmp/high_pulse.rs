#[doc = "Register `HIGH_PULSE` reader"]
pub type R = crate::R<HighPulseSpec>;
#[doc = "Register `HIGH_PULSE` writer"]
pub type W = crate::W<HighPulseSpec>;
#[doc = "Field `H_PULSE_CNT` reader - High pulse Count"]
pub type HPulseCntR = crate::FieldReader;
#[doc = "Field `H_PULSE_CNT` writer - High pulse Count"]
pub type HPulseCntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - High pulse Count"]
    #[inline(always)]
    pub fn h_pulse_cnt(&self) -> HPulseCntR {
        HPulseCntR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - High pulse Count"]
    #[inline(always)]
    pub fn h_pulse_cnt(&mut self) -> HPulseCntW<'_, HighPulseSpec> {
        HPulseCntW::new(self, 0)
    }
}
#[doc = "HIGH_PULSE\n\nYou can [`read`](crate::Reg::read) this register and get [`high_pulse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`high_pulse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HighPulseSpec;
impl crate::RegisterSpec for HighPulseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`high_pulse::R`](R) reader structure"]
impl crate::Readable for HighPulseSpec {}
#[doc = "`write(|w| ..)` method takes [`high_pulse::W`](W) writer structure"]
impl crate::Writable for HighPulseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HIGH_PULSE to value 0"]
impl crate::Resettable for HighPulseSpec {}
