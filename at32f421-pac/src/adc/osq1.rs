#[doc = "Register `OSQ1` reader"]
pub type R = crate::R<Osq1Spec>;
#[doc = "Register `OSQ1` writer"]
pub type W = crate::W<Osq1Spec>;
#[doc = "Field `OSN13` reader - Number of 13th conversion in ordinary sequence"]
pub type Osn13R = crate::FieldReader;
#[doc = "Field `OSN13` writer - Number of 13th conversion in ordinary sequence"]
pub type Osn13W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN14` reader - Number of 14th conversion in ordinary sequence"]
pub type Osn14R = crate::FieldReader;
#[doc = "Field `OSN14` writer - Number of 14th conversion in ordinary sequence"]
pub type Osn14W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN15` reader - Number of 15th conversion in ordinary sequence"]
pub type Osn15R = crate::FieldReader;
#[doc = "Field `OSN15` writer - Number of 15th conversion in ordinary sequence"]
pub type Osn15W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN16` reader - Number of 16th conversion in ordinary sequence"]
pub type Osn16R = crate::FieldReader;
#[doc = "Field `OSN16` writer - Number of 16th conversion in ordinary sequence"]
pub type Osn16W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OCLEN` reader - Ordinary conversion sequence length"]
pub type OclenR = crate::FieldReader;
#[doc = "Field `OCLEN` writer - Ordinary conversion sequence length"]
pub type OclenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - Number of 13th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn13(&self) -> Osn13R {
        Osn13R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Number of 14th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn14(&self) -> Osn14R {
        Osn14R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Number of 15th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn15(&self) -> Osn15R {
        Osn15R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Number of 16th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn16(&self) -> Osn16R {
        Osn16R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:23 - Ordinary conversion sequence length"]
    #[inline(always)]
    pub fn oclen(&self) -> OclenR {
        OclenR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of 13th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn13(&mut self) -> Osn13W<'_, Osq1Spec> {
        Osn13W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Number of 14th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn14(&mut self) -> Osn14W<'_, Osq1Spec> {
        Osn14W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Number of 15th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn15(&mut self) -> Osn15W<'_, Osq1Spec> {
        Osn15W::new(self, 10)
    }
    #[doc = "Bits 15:19 - Number of 16th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn16(&mut self) -> Osn16W<'_, Osq1Spec> {
        Osn16W::new(self, 15)
    }
    #[doc = "Bits 20:23 - Ordinary conversion sequence length"]
    #[inline(always)]
    pub fn oclen(&mut self) -> OclenW<'_, Osq1Spec> {
        OclenW::new(self, 20)
    }
}
#[doc = "Ordinary sequence register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`osq1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osq1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Osq1Spec;
impl crate::RegisterSpec for Osq1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osq1::R`](R) reader structure"]
impl crate::Readable for Osq1Spec {}
#[doc = "`write(|w| ..)` method takes [`osq1::W`](W) writer structure"]
impl crate::Writable for Osq1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSQ1 to value 0"]
impl crate::Resettable for Osq1Spec {}
