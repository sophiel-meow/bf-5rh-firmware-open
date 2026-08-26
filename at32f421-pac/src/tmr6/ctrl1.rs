#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `TMREN` reader - TMR enable"]
pub type TmrenR = crate::BitReader;
#[doc = "Field `TMREN` writer - TMR enable"]
pub type TmrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFEN` reader - Overflow event enable"]
pub type OvfenR = crate::BitReader;
#[doc = "Field `OVFEN` writer - Overflow event enable"]
pub type OvfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFS` reader - Overflow event source"]
pub type OvfsR = crate::BitReader;
#[doc = "Field `OVFS` writer - Overflow event source"]
pub type OvfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCMEN` reader - One cycle mode enable"]
pub type OcmenR = crate::BitReader;
#[doc = "Field `OCMEN` writer - One cycle mode enable"]
pub type OcmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRBEN` reader - Period buffer enable"]
pub type PrbenR = crate::BitReader;
#[doc = "Field `PRBEN` writer - Period buffer enable"]
pub type PrbenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TMR enable"]
    #[inline(always)]
    pub fn tmren(&self) -> TmrenR {
        TmrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overflow event enable"]
    #[inline(always)]
    pub fn ovfen(&self) -> OvfenR {
        OvfenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow event source"]
    #[inline(always)]
    pub fn ovfs(&self) -> OvfsR {
        OvfsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - One cycle mode enable"]
    #[inline(always)]
    pub fn ocmen(&self) -> OcmenR {
        OcmenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Period buffer enable"]
    #[inline(always)]
    pub fn prben(&self) -> PrbenR {
        PrbenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TMR enable"]
    #[inline(always)]
    pub fn tmren(&mut self) -> TmrenW<'_, Ctrl1Spec> {
        TmrenW::new(self, 0)
    }
    #[doc = "Bit 1 - Overflow event enable"]
    #[inline(always)]
    pub fn ovfen(&mut self) -> OvfenW<'_, Ctrl1Spec> {
        OvfenW::new(self, 1)
    }
    #[doc = "Bit 2 - Overflow event source"]
    #[inline(always)]
    pub fn ovfs(&mut self) -> OvfsW<'_, Ctrl1Spec> {
        OvfsW::new(self, 2)
    }
    #[doc = "Bit 3 - One cycle mode enable"]
    #[inline(always)]
    pub fn ocmen(&mut self) -> OcmenW<'_, Ctrl1Spec> {
        OcmenW::new(self, 3)
    }
    #[doc = "Bit 7 - Period buffer enable"]
    #[inline(always)]
    pub fn prben(&mut self) -> PrbenW<'_, Ctrl1Spec> {
        PrbenW::new(self, 7)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for Ctrl1Spec {}
