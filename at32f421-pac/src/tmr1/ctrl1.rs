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
#[doc = "Field `OWCDIR` reader - One-way count direction"]
pub type OwcdirR = crate::BitReader;
#[doc = "Field `OWCDIR` writer - One-way count direction"]
pub type OwcdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWCMSEL` reader - Two-way count mode selection"]
pub type TwcmselR = crate::FieldReader;
#[doc = "Field `TWCMSEL` writer - Two-way count mode selection"]
pub type TwcmselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRBEN` reader - Period buffer enable"]
pub type PrbenR = crate::BitReader;
#[doc = "Field `PRBEN` writer - Period buffer enable"]
pub type PrbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDIV` reader - Clock divider"]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Clock divider"]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    #[doc = "Bit 4 - One-way count direction"]
    #[inline(always)]
    pub fn owcdir(&self) -> OwcdirR {
        OwcdirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Two-way count mode selection"]
    #[inline(always)]
    pub fn twcmsel(&self) -> TwcmselR {
        TwcmselR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Period buffer enable"]
    #[inline(always)]
    pub fn prben(&self) -> PrbenR {
        PrbenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock divider"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 8) & 3) as u8)
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
    #[doc = "Bit 4 - One-way count direction"]
    #[inline(always)]
    pub fn owcdir(&mut self) -> OwcdirW<'_, Ctrl1Spec> {
        OwcdirW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Two-way count mode selection"]
    #[inline(always)]
    pub fn twcmsel(&mut self) -> TwcmselW<'_, Ctrl1Spec> {
        TwcmselW::new(self, 5)
    }
    #[doc = "Bit 7 - Period buffer enable"]
    #[inline(always)]
    pub fn prben(&mut self) -> PrbenW<'_, Ctrl1Spec> {
        PrbenW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Clock divider"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<'_, Ctrl1Spec> {
        ClkdivW::new(self, 8)
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
