#[doc = "Register `SBS` reader"]
pub type R = crate::R<SbsSpec>;
#[doc = "Field `SBS` reader - Sub second value"]
pub type SbsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Sub second value"]
    #[inline(always)]
    pub fn sbs(&self) -> SbsR {
        SbsR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "sub second register\n\nYou can [`read`](crate::Reg::read) this register and get [`sbs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SbsSpec;
impl crate::RegisterSpec for SbsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbs::R`](R) reader structure"]
impl crate::Readable for SbsSpec {}
#[doc = "`reset()` method sets SBS to value 0"]
impl crate::Resettable for SbsSpec {}
