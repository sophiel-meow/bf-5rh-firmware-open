#[doc = "Register `CDT` reader"]
pub type R = crate::R<CdtSpec>;
#[doc = "Register `CDT` writer"]
pub type W = crate::W<CdtSpec>;
#[doc = "Field `CDT` reader - Common Data"]
pub type CdtR = crate::BitReader;
#[doc = "Field `CDT` writer - Common Data"]
pub type CdtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Common Data"]
    #[inline(always)]
    pub fn cdt(&self) -> CdtR {
        CdtR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Common Data"]
    #[inline(always)]
    pub fn cdt(&mut self) -> CdtW<'_, CdtSpec> {
        CdtW::new(self, 0)
    }
}
#[doc = "Common data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdtSpec;
impl crate::RegisterSpec for CdtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdt::R`](R) reader structure"]
impl crate::Readable for CdtSpec {}
#[doc = "`write(|w| ..)` method takes [`cdt::W`](W) writer structure"]
impl crate::Writable for CdtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CDT to value 0"]
impl crate::Resettable for CdtSpec {}
