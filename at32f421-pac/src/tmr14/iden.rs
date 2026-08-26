#[doc = "Register `IDEN` reader"]
pub type R = crate::R<IdenSpec>;
#[doc = "Register `IDEN` writer"]
pub type W = crate::W<IdenSpec>;
#[doc = "Field `OVFIEN` reader - Overflow interrupt enable"]
pub type OvfienR = crate::BitReader;
#[doc = "Field `OVFIEN` writer - Overflow interrupt enable"]
pub type OvfienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1IEN` reader - Channel 1 interrupt enable"]
pub type C1ienR = crate::BitReader;
#[doc = "Field `C1IEN` writer - Channel 1 interrupt enable"]
pub type C1ienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovfien(&self) -> OvfienR {
        OvfienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 interrupt enable"]
    #[inline(always)]
    pub fn c1ien(&self) -> C1ienR {
        C1ienR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovfien(&mut self) -> OvfienW<'_, IdenSpec> {
        OvfienW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 interrupt enable"]
    #[inline(always)]
    pub fn c1ien(&mut self) -> C1ienW<'_, IdenSpec> {
        C1ienW::new(self, 1)
    }
}
#[doc = "Interrupt/DMA enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`iden::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iden::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdenSpec;
impl crate::RegisterSpec for IdenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iden::R`](R) reader structure"]
impl crate::Readable for IdenSpec {}
#[doc = "`write(|w| ..)` method takes [`iden::W`](W) writer structure"]
impl crate::Writable for IdenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDEN to value 0"]
impl crate::Resettable for IdenSpec {}
