#[doc = "Register `EXINTC2` reader"]
pub type R = crate::R<Exintc2Spec>;
#[doc = "Register `EXINTC2` writer"]
pub type W = crate::W<Exintc2Spec>;
#[doc = "Field `EXINT4` reader - EXINT 4 configuration bits"]
pub type Exint4R = crate::FieldReader;
#[doc = "Field `EXINT4` writer - EXINT 4 configuration bits"]
pub type Exint4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT5` reader - EXINT 5 configuration bits"]
pub type Exint5R = crate::FieldReader;
#[doc = "Field `EXINT5` writer - EXINT 5 configuration bits"]
pub type Exint5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT6` reader - EXINT 6 configuration bits"]
pub type Exint6R = crate::FieldReader;
#[doc = "Field `EXINT6` writer - EXINT 6 configuration bits"]
pub type Exint6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT7` reader - EXINT 7 configuration bits"]
pub type Exint7R = crate::FieldReader;
#[doc = "Field `EXINT7` writer - EXINT 7 configuration bits"]
pub type Exint7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - EXINT 4 configuration bits"]
    #[inline(always)]
    pub fn exint4(&self) -> Exint4R {
        Exint4R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXINT 5 configuration bits"]
    #[inline(always)]
    pub fn exint5(&self) -> Exint5R {
        Exint5R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXINT 6 configuration bits"]
    #[inline(always)]
    pub fn exint6(&self) -> Exint6R {
        Exint6R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXINT 7 configuration bits"]
    #[inline(always)]
    pub fn exint7(&self) -> Exint7R {
        Exint7R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXINT 4 configuration bits"]
    #[inline(always)]
    pub fn exint4(&mut self) -> Exint4W<'_, Exintc2Spec> {
        Exint4W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXINT 5 configuration bits"]
    #[inline(always)]
    pub fn exint5(&mut self) -> Exint5W<'_, Exintc2Spec> {
        Exint5W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXINT 6 configuration bits"]
    #[inline(always)]
    pub fn exint6(&mut self) -> Exint6W<'_, Exintc2Spec> {
        Exint6W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXINT 7 configuration bits"]
    #[inline(always)]
    pub fn exint7(&mut self) -> Exint7W<'_, Exintc2Spec> {
        Exint7W::new(self, 12)
    }
}
#[doc = "external interrupt configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`exintc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exintc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Exintc2Spec;
impl crate::RegisterSpec for Exintc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc2::R`](R) reader structure"]
impl crate::Readable for Exintc2Spec {}
#[doc = "`write(|w| ..)` method takes [`exintc2::W`](W) writer structure"]
impl crate::Writable for Exintc2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXINTC2 to value 0"]
impl crate::Resettable for Exintc2Spec {}
