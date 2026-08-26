#[doc = "Register `UNLOCK` writer"]
pub type W = crate::W<UnlockSpec>;
#[doc = "Field `UKVAL` writer - Unlock key value"]
pub type UkvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Unlock key value"]
    #[inline(always)]
    pub fn ukval(&mut self) -> UkvalW<'_, UnlockSpec> {
        UkvalW::new(self, 0)
    }
}
#[doc = "Unlock register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unlock::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UnlockSpec;
impl crate::RegisterSpec for UnlockSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`unlock::W`](W) writer structure"]
impl crate::Writable for UnlockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNLOCK to value 0"]
impl crate::Resettable for UnlockSpec {}
