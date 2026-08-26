#[doc = "Register `OSQ2` reader"]
pub type R = crate::R<Osq2Spec>;
#[doc = "Register `OSQ2` writer"]
pub type W = crate::W<Osq2Spec>;
#[doc = "Field `OSN7` reader - Number of 13th conversion in ordinary sequence"]
pub type Osn7R = crate::FieldReader;
#[doc = "Field `OSN7` writer - Number of 13th conversion in ordinary sequence"]
pub type Osn7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN8` reader - Number of 7th conversion in ordinary sequence"]
pub type Osn8R = crate::FieldReader;
#[doc = "Field `OSN8` writer - Number of 7th conversion in ordinary sequence"]
pub type Osn8W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN9` reader - Number of 8th conversion in ordinary sequence"]
pub type Osn9R = crate::FieldReader;
#[doc = "Field `OSN9` writer - Number of 8th conversion in ordinary sequence"]
pub type Osn9W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN10` reader - Number of 10th conversion in ordinary sequence"]
pub type Osn10R = crate::FieldReader;
#[doc = "Field `OSN10` writer - Number of 10th conversion in ordinary sequence"]
pub type Osn10W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN11` reader - Number of 11th conversion in ordinary sequence"]
pub type Osn11R = crate::FieldReader;
#[doc = "Field `OSN11` writer - Number of 11th conversion in ordinary sequence"]
pub type Osn11W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OSN12` reader - Number of 12th conversion in ordinary sequence"]
pub type Osn12R = crate::FieldReader;
#[doc = "Field `OSN12` writer - Number of 12th conversion in ordinary sequence"]
pub type Osn12W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Number of 13th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn7(&self) -> Osn7R {
        Osn7R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Number of 7th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn8(&self) -> Osn8R {
        Osn8R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Number of 8th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn9(&self) -> Osn9R {
        Osn9R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Number of 10th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn10(&self) -> Osn10R {
        Osn10R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Number of 11th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn11(&self) -> Osn11R {
        Osn11R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Number of 12th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn12(&self) -> Osn12R {
        Osn12R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of 13th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn7(&mut self) -> Osn7W<'_, Osq2Spec> {
        Osn7W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Number of 7th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn8(&mut self) -> Osn8W<'_, Osq2Spec> {
        Osn8W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Number of 8th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn9(&mut self) -> Osn9W<'_, Osq2Spec> {
        Osn9W::new(self, 10)
    }
    #[doc = "Bits 15:19 - Number of 10th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn10(&mut self) -> Osn10W<'_, Osq2Spec> {
        Osn10W::new(self, 15)
    }
    #[doc = "Bits 20:24 - Number of 11th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn11(&mut self) -> Osn11W<'_, Osq2Spec> {
        Osn11W::new(self, 20)
    }
    #[doc = "Bits 25:29 - Number of 12th conversion in ordinary sequence"]
    #[inline(always)]
    pub fn osn12(&mut self) -> Osn12W<'_, Osq2Spec> {
        Osn12W::new(self, 25)
    }
}
#[doc = "Ordinary sequence register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`osq2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osq2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Osq2Spec;
impl crate::RegisterSpec for Osq2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osq2::R`](R) reader structure"]
impl crate::Readable for Osq2Spec {}
#[doc = "`write(|w| ..)` method takes [`osq2::W`](W) writer structure"]
impl crate::Writable for Osq2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSQ2 to value 0"]
impl crate::Resettable for Osq2Spec {}
