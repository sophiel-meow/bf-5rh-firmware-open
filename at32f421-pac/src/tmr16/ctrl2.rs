#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Field `CBCTRL` reader - Channel buffer control"]
pub type CbctrlR = crate::BitReader;
#[doc = "Field `CBCTRL` writer - Channel buffer control"]
pub type CbctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCFS` reader - Channel control bit fresh select"]
pub type CcfsR = crate::BitReader;
#[doc = "Field `CCFS` writer - Channel control bit fresh select"]
pub type CcfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRS` reader - DMA request source"]
pub type DrsR = crate::BitReader;
#[doc = "Field `DRS` writer - DMA request source"]
pub type DrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1IOS` reader - Channel 1 idle output state"]
pub type C1iosR = crate::BitReader;
#[doc = "Field `C1IOS` writer - Channel 1 idle output state"]
pub type C1iosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1CIOS` reader - Channel 1 complementary idle output state"]
pub type C1ciosR = crate::BitReader;
#[doc = "Field `C1CIOS` writer - Channel 1 complementary idle output state"]
pub type C1ciosW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel buffer control"]
    #[inline(always)]
    pub fn cbctrl(&self) -> CbctrlR {
        CbctrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Channel control bit fresh select"]
    #[inline(always)]
    pub fn ccfs(&self) -> CcfsR {
        CcfsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA request source"]
    #[inline(always)]
    pub fn drs(&self) -> DrsR {
        DrsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 1 idle output state"]
    #[inline(always)]
    pub fn c1ios(&self) -> C1iosR {
        C1iosR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 complementary idle output state"]
    #[inline(always)]
    pub fn c1cios(&self) -> C1ciosR {
        C1ciosR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel buffer control"]
    #[inline(always)]
    pub fn cbctrl(&mut self) -> CbctrlW<'_, Ctrl2Spec> {
        CbctrlW::new(self, 0)
    }
    #[doc = "Bit 2 - Channel control bit fresh select"]
    #[inline(always)]
    pub fn ccfs(&mut self) -> CcfsW<'_, Ctrl2Spec> {
        CcfsW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA request source"]
    #[inline(always)]
    pub fn drs(&mut self) -> DrsW<'_, Ctrl2Spec> {
        DrsW::new(self, 3)
    }
    #[doc = "Bit 8 - Channel 1 idle output state"]
    #[inline(always)]
    pub fn c1ios(&mut self) -> C1iosW<'_, Ctrl2Spec> {
        C1iosW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 1 complementary idle output state"]
    #[inline(always)]
    pub fn c1cios(&mut self) -> C1ciosW<'_, Ctrl2Spec> {
        C1ciosW::new(self, 9)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl2Spec;
impl crate::RegisterSpec for Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for Ctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for Ctrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for Ctrl2Spec {}
