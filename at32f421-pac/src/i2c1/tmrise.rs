#[doc = "Register `TMRISE` reader"]
pub type R = crate::R<TmriseSpec>;
#[doc = "Register `TMRISE` writer"]
pub type W = crate::W<TmriseSpec>;
#[doc = "Field `RISETIME` reader - I2C bus rise time"]
pub type RisetimeR = crate::FieldReader;
#[doc = "Field `RISETIME` writer - I2C bus rise time"]
pub type RisetimeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - I2C bus rise time"]
    #[inline(always)]
    pub fn risetime(&self) -> RisetimeR {
        RisetimeR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I2C bus rise time"]
    #[inline(always)]
    pub fn risetime(&mut self) -> RisetimeW<'_, TmriseSpec> {
        RisetimeW::new(self, 0)
    }
}
#[doc = "TRISE register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmrise::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmrise::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmriseSpec;
impl crate::RegisterSpec for TmriseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmrise::R`](R) reader structure"]
impl crate::Readable for TmriseSpec {}
#[doc = "`write(|w| ..)` method takes [`tmrise::W`](W) writer structure"]
impl crate::Writable for TmriseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMRISE to value 0x02"]
impl crate::Resettable for TmriseSpec {
    const RESET_VALUE: u32 = 0x02;
}
