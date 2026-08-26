#[doc = "Register `ODT` reader"]
pub type R = crate::R<OdtSpec>;
#[doc = "Field `ODT` reader - Conversion data of ordinary channel"]
pub type OdtR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Conversion data of ordinary channel"]
    #[inline(always)]
    pub fn odt(&self) -> OdtR {
        OdtR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Ordinary data register\n\nYou can [`read`](crate::Reg::read) this register and get [`odt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OdtSpec;
impl crate::RegisterSpec for OdtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odt::R`](R) reader structure"]
impl crate::Readable for OdtSpec {}
#[doc = "`reset()` method sets ODT to value 0"]
impl crate::Resettable for OdtSpec {}
