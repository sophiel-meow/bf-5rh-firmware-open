#[doc = "Register `SLIB_SET_RANGE` writer"]
pub type W = crate::W<SlibSetRangeSpec>;
#[doc = "Field `SLIB_SS_SET` writer - sLib start sector setting"]
pub type SlibSsSetW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SLIB_ISS_SET` writer - sLib instruction start sector setting"]
pub type SlibIssSetW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SLIB_ES_SET` writer - sLib end sector setting"]
pub type SlibEsSetW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl W {
    #[doc = "Bits 0:10 - sLib start sector setting"]
    #[inline(always)]
    pub fn slib_ss_set(&mut self) -> SlibSsSetW<'_, SlibSetRangeSpec> {
        SlibSsSetW::new(self, 0)
    }
    #[doc = "Bits 11:21 - sLib instruction start sector setting"]
    #[inline(always)]
    pub fn slib_iss_set(&mut self) -> SlibIssSetW<'_, SlibSetRangeSpec> {
        SlibIssSetW::new(self, 11)
    }
    #[doc = "Bits 22:31 - sLib end sector setting"]
    #[inline(always)]
    pub fn slib_es_set(&mut self) -> SlibEsSetW<'_, SlibSetRangeSpec> {
        SlibEsSetW::new(self, 22)
    }
}
#[doc = "Configure sLib range register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slib_set_range::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlibSetRangeSpec;
impl crate::RegisterSpec for SlibSetRangeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`slib_set_range::W`](W) writer structure"]
impl crate::Writable for SlibSetRangeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLIB_SET_RANGE to value 0"]
impl crate::Resettable for SlibSetRangeSpec {}
