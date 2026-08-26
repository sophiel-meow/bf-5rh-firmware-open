#[doc = "Register `CTRLSTS` reader"]
pub type R = crate::R<CtrlstsSpec>;
#[doc = "Register `CTRLSTS` writer"]
pub type W = crate::W<CtrlstsSpec>;
#[doc = "Field `SWEF` reader - Standby wake-up event flag"]
pub type SwefR = crate::BitReader;
#[doc = "Field `SEF` reader - Standby mode entry flag"]
pub type SefR = crate::BitReader;
#[doc = "Field `PVMOF` reader - Power voltage monitoring output flag"]
pub type PvmofR = crate::BitReader;
#[doc = "Field `SWPEN1` reader - Standby wake-up pin1 enable"]
pub type Swpen1R = crate::BitReader;
#[doc = "Field `SWPEN1` writer - Standby wake-up pin1 enable"]
pub type Swpen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWPEN2` reader - Standby wake-up pin2 enable"]
pub type Swpen2R = crate::BitReader;
#[doc = "Field `SWPEN2` writer - Standby wake-up pin2 enable"]
pub type Swpen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWPEN6` reader - Standby wake-up pin2 enable"]
pub type Swpen6R = crate::BitReader;
#[doc = "Field `SWPEN6` writer - Standby wake-up pin2 enable"]
pub type Swpen6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWPEN7` reader - Standby wake-up pin2 enable"]
pub type Swpen7R = crate::BitReader;
#[doc = "Field `SWPEN7` writer - Standby wake-up pin2 enable"]
pub type Swpen7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Standby wake-up event flag"]
    #[inline(always)]
    pub fn swef(&self) -> SwefR {
        SwefR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby mode entry flag"]
    #[inline(always)]
    pub fn sef(&self) -> SefR {
        SefR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power voltage monitoring output flag"]
    #[inline(always)]
    pub fn pvmof(&self) -> PvmofR {
        PvmofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Standby wake-up pin1 enable"]
    #[inline(always)]
    pub fn swpen1(&self) -> Swpen1R {
        Swpen1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Standby wake-up pin2 enable"]
    #[inline(always)]
    pub fn swpen2(&self) -> Swpen2R {
        Swpen2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Standby wake-up pin2 enable"]
    #[inline(always)]
    pub fn swpen6(&self) -> Swpen6R {
        Swpen6R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Standby wake-up pin2 enable"]
    #[inline(always)]
    pub fn swpen7(&self) -> Swpen7R {
        Swpen7R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Standby wake-up pin1 enable"]
    #[inline(always)]
    pub fn swpen1(&mut self) -> Swpen1W<'_, CtrlstsSpec> {
        Swpen1W::new(self, 8)
    }
    #[doc = "Bit 9 - Standby wake-up pin2 enable"]
    #[inline(always)]
    pub fn swpen2(&mut self) -> Swpen2W<'_, CtrlstsSpec> {
        Swpen2W::new(self, 9)
    }
    #[doc = "Bit 13 - Standby wake-up pin2 enable"]
    #[inline(always)]
    pub fn swpen6(&mut self) -> Swpen6W<'_, CtrlstsSpec> {
        Swpen6W::new(self, 13)
    }
    #[doc = "Bit 14 - Standby wake-up pin2 enable"]
    #[inline(always)]
    pub fn swpen7(&mut self) -> Swpen7W<'_, CtrlstsSpec> {
        Swpen7W::new(self, 14)
    }
}
#[doc = "Power control and status register (PWC_CTRLSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlstsSpec;
impl crate::RegisterSpec for CtrlstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlsts::R`](R) reader structure"]
impl crate::Readable for CtrlstsSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlsts::W`](W) writer structure"]
impl crate::Writable for CtrlstsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLSTS to value 0"]
impl crate::Resettable for CtrlstsSpec {}
