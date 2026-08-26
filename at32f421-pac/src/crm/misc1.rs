#[doc = "Register `MISC1` reader"]
pub type R = crate::R<Misc1Spec>;
#[doc = "Register `MISC1` writer"]
pub type W = crate::W<Misc1Spec>;
#[doc = "Field `HICKCAL_KEY` reader - HICKCAL write key value"]
pub type HickcalKeyR = crate::FieldReader;
#[doc = "Field `HICKCAL_KEY` writer - HICKCAL write key value"]
pub type HickcalKeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLKOUT_SEL3` reader - Clock output bit3"]
pub type ClkoutSel3R = crate::BitReader;
#[doc = "Field `CLKOUT_SEL3` writer - Clock output bit3"]
pub type ClkoutSel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HICKDIV` reader - HICK 6 divider selection"]
pub type HickdivR = crate::BitReader;
#[doc = "Field `HICKDIV` writer - HICK 6 divider selection"]
pub type HickdivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKOUTDIV` reader - Clock output division"]
pub type ClkoutdivR = crate::FieldReader;
#[doc = "Field `CLKOUTDIV` writer - Clock output division"]
pub type ClkoutdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - HICKCAL write key value"]
    #[inline(always)]
    pub fn hickcal_key(&self) -> HickcalKeyR {
        HickcalKeyR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Clock output bit3"]
    #[inline(always)]
    pub fn clkout_sel3(&self) -> ClkoutSel3R {
        ClkoutSel3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 25 - HICK 6 divider selection"]
    #[inline(always)]
    pub fn hickdiv(&self) -> HickdivR {
        HickdivR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Clock output division"]
    #[inline(always)]
    pub fn clkoutdiv(&self) -> ClkoutdivR {
        ClkoutdivR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - HICKCAL write key value"]
    #[inline(always)]
    pub fn hickcal_key(&mut self) -> HickcalKeyW<'_, Misc1Spec> {
        HickcalKeyW::new(self, 0)
    }
    #[doc = "Bit 16 - Clock output bit3"]
    #[inline(always)]
    pub fn clkout_sel3(&mut self) -> ClkoutSel3W<'_, Misc1Spec> {
        ClkoutSel3W::new(self, 16)
    }
    #[doc = "Bit 25 - HICK 6 divider selection"]
    #[inline(always)]
    pub fn hickdiv(&mut self) -> HickdivW<'_, Misc1Spec> {
        HickdivW::new(self, 25)
    }
    #[doc = "Bits 28:31 - Clock output division"]
    #[inline(always)]
    pub fn clkoutdiv(&mut self) -> ClkoutdivW<'_, Misc1Spec> {
        ClkoutdivW::new(self, 28)
    }
}
#[doc = "Miscellaneous register1\n\nYou can [`read`](crate::Reg::read) this register and get [`misc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Misc1Spec;
impl crate::RegisterSpec for Misc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc1::R`](R) reader structure"]
impl crate::Readable for Misc1Spec {}
#[doc = "`write(|w| ..)` method takes [`misc1::W`](W) writer structure"]
impl crate::Writable for Misc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISC1 to value 0"]
impl crate::Resettable for Misc1Spec {}
