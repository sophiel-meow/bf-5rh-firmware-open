#[doc = "Register `BTM_MODE_SET` writer"]
pub type W = crate::W<BtmModeSetSpec>;
#[doc = "Field `BTM_MODE_SET` writer - Boot memory mode setting"]
pub type BtmModeSetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Boot memory mode setting"]
    #[inline(always)]
    pub fn btm_mode_set(&mut self) -> BtmModeSetW<'_, BtmModeSetSpec> {
        BtmModeSetW::new(self, 0)
    }
}
#[doc = "Boot memory mode setting register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btm_mode_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BtmModeSetSpec;
impl crate::RegisterSpec for BtmModeSetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`btm_mode_set::W`](W) writer structure"]
impl crate::Writable for BtmModeSetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BTM_MODE_SET to value 0"]
impl crate::Resettable for BtmModeSetSpec {}
