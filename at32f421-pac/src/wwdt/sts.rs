#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<StsSpec>;
#[doc = "Field `RLDF` reader - Reload counter interrupt flag"]
pub type RldfR = crate::BitReader;
#[doc = "Field `RLDF` writer - Reload counter interrupt flag"]
pub type RldfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reload counter interrupt flag"]
    #[inline(always)]
    pub fn rldf(&self) -> RldfR {
        RldfR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reload counter interrupt flag"]
    #[inline(always)]
    pub fn rldf(&mut self) -> RldfW<'_, StsSpec> {
        RldfW::new(self, 0)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for StsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for StsSpec {}
