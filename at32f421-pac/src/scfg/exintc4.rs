#[doc = "Register `EXINTC4` reader"]
pub type R = crate::R<Exintc4Spec>;
#[doc = "Register `EXINTC4` writer"]
pub type W = crate::W<Exintc4Spec>;
#[doc = "Field `EXINT12` reader - EXINT 12 configuration bits"]
pub type Exint12R = crate::FieldReader;
#[doc = "Field `EXINT12` writer - EXINT 12 configuration bits"]
pub type Exint12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT13` reader - EXINT 13 configuration bits"]
pub type Exint13R = crate::FieldReader;
#[doc = "Field `EXINT13` writer - EXINT 13 configuration bits"]
pub type Exint13W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT14` reader - EXINT 14 configuration bits"]
pub type Exint14R = crate::FieldReader;
#[doc = "Field `EXINT14` writer - EXINT 14 configuration bits"]
pub type Exint14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT15` reader - EXINT 15 configuration bits"]
pub type Exint15R = crate::FieldReader;
#[doc = "Field `EXINT15` writer - EXINT 15 configuration bits"]
pub type Exint15W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - EXINT 12 configuration bits"]
    #[inline(always)]
    pub fn exint12(&self) -> Exint12R {
        Exint12R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXINT 13 configuration bits"]
    #[inline(always)]
    pub fn exint13(&self) -> Exint13R {
        Exint13R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXINT 14 configuration bits"]
    #[inline(always)]
    pub fn exint14(&self) -> Exint14R {
        Exint14R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXINT 15 configuration bits"]
    #[inline(always)]
    pub fn exint15(&self) -> Exint15R {
        Exint15R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXINT 12 configuration bits"]
    #[inline(always)]
    pub fn exint12(&mut self) -> Exint12W<'_, Exintc4Spec> {
        Exint12W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXINT 13 configuration bits"]
    #[inline(always)]
    pub fn exint13(&mut self) -> Exint13W<'_, Exintc4Spec> {
        Exint13W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXINT 14 configuration bits"]
    #[inline(always)]
    pub fn exint14(&mut self) -> Exint14W<'_, Exintc4Spec> {
        Exint14W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXINT 15 configuration bits"]
    #[inline(always)]
    pub fn exint15(&mut self) -> Exint15W<'_, Exintc4Spec> {
        Exint15W::new(self, 12)
    }
}
#[doc = "external interrupt configuration register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`exintc4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exintc4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Exintc4Spec;
impl crate::RegisterSpec for Exintc4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc4::R`](R) reader structure"]
impl crate::Readable for Exintc4Spec {}
#[doc = "`write(|w| ..)` method takes [`exintc4::W`](W) writer structure"]
impl crate::Writable for Exintc4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXINTC4 to value 0"]
impl crate::Resettable for Exintc4Spec {}
