#[doc = "Register `SPT2` reader"]
pub type R = crate::R<Spt2Spec>;
#[doc = "Register `SPT2` writer"]
pub type W = crate::W<Spt2Spec>;
#[doc = "Field `CSPT0` reader - Selection sample time of channel ADC_IN0"]
pub type Cspt0R = crate::FieldReader;
#[doc = "Field `CSPT0` writer - Selection sample time of channel ADC_IN0"]
pub type Cspt0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSPT1` reader - Selection sample time of channel ADC_IN1"]
pub type Cspt1R = crate::FieldReader;
#[doc = "Field `CSPT1` writer - Selection sample time of channel ADC_IN1"]
pub type Cspt1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSPT2` reader - Selection sample time of channel ADC_IN2"]
pub type Cspt2R = crate::FieldReader;
#[doc = "Field `CSPT2` writer - Selection sample time of channel ADC_IN2"]
pub type Cspt2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSPT3` reader - Selection sample time of channel ADC_IN3"]
pub type Cspt3R = crate::FieldReader;
#[doc = "Field `CSPT3` writer - Selection sample time of channel ADC_IN3"]
pub type Cspt3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSPT4` reader - Selection sample time of channel ADC_IN4"]
pub type Cspt4R = crate::FieldReader;
#[doc = "Field `CSPT4` writer - Selection sample time of channel ADC_IN4"]
pub type Cspt4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSPT5` reader - Selection sample time of channel ADC_IN5"]
pub type Cspt5R = crate::FieldReader;
#[doc = "Field `CSPT5` writer - Selection sample time of channel ADC_IN5"]
pub type Cspt5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSPT6` reader - Selection sample time of channel ADC_IN6"]
pub type Cspt6R = crate::FieldReader;
#[doc = "Field `CSPT6` writer - Selection sample time of channel ADC_IN6"]
pub type Cspt6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSPT7` reader - Selection sample time of channel ADC_IN7"]
pub type Cspt7R = crate::FieldReader;
#[doc = "Field `CSPT7` writer - Selection sample time of channel ADC_IN7"]
pub type Cspt7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSPT8` reader - Selection sample time of channel ADC_IN8"]
pub type Cspt8R = crate::FieldReader;
#[doc = "Field `CSPT8` writer - Selection sample time of channel ADC_IN8"]
pub type Cspt8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSPT9` reader - Selection sample time of channel ADC_IN9"]
pub type Cspt9R = crate::FieldReader;
#[doc = "Field `CSPT9` writer - Selection sample time of channel ADC_IN9"]
pub type Cspt9W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN0"]
    #[inline(always)]
    pub fn cspt0(&self) -> Cspt0R {
        Cspt0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN1"]
    #[inline(always)]
    pub fn cspt1(&self) -> Cspt1R {
        Cspt1R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Selection sample time of channel ADC_IN2"]
    #[inline(always)]
    pub fn cspt2(&self) -> Cspt2R {
        Cspt2R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Selection sample time of channel ADC_IN3"]
    #[inline(always)]
    pub fn cspt3(&self) -> Cspt3R {
        Cspt3R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Selection sample time of channel ADC_IN4"]
    #[inline(always)]
    pub fn cspt4(&self) -> Cspt4R {
        Cspt4R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Selection sample time of channel ADC_IN5"]
    #[inline(always)]
    pub fn cspt5(&self) -> Cspt5R {
        Cspt5R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Selection sample time of channel ADC_IN6"]
    #[inline(always)]
    pub fn cspt6(&self) -> Cspt6R {
        Cspt6R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Selection sample time of channel ADC_IN7"]
    #[inline(always)]
    pub fn cspt7(&self) -> Cspt7R {
        Cspt7R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Selection sample time of channel ADC_IN8"]
    #[inline(always)]
    pub fn cspt8(&self) -> Cspt8R {
        Cspt8R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Selection sample time of channel ADC_IN9"]
    #[inline(always)]
    pub fn cspt9(&self) -> Cspt9R {
        Cspt9R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN0"]
    #[inline(always)]
    pub fn cspt0(&mut self) -> Cspt0W<'_, Spt2Spec> {
        Cspt0W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN1"]
    #[inline(always)]
    pub fn cspt1(&mut self) -> Cspt1W<'_, Spt2Spec> {
        Cspt1W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Selection sample time of channel ADC_IN2"]
    #[inline(always)]
    pub fn cspt2(&mut self) -> Cspt2W<'_, Spt2Spec> {
        Cspt2W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Selection sample time of channel ADC_IN3"]
    #[inline(always)]
    pub fn cspt3(&mut self) -> Cspt3W<'_, Spt2Spec> {
        Cspt3W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Selection sample time of channel ADC_IN4"]
    #[inline(always)]
    pub fn cspt4(&mut self) -> Cspt4W<'_, Spt2Spec> {
        Cspt4W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Selection sample time of channel ADC_IN5"]
    #[inline(always)]
    pub fn cspt5(&mut self) -> Cspt5W<'_, Spt2Spec> {
        Cspt5W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Selection sample time of channel ADC_IN6"]
    #[inline(always)]
    pub fn cspt6(&mut self) -> Cspt6W<'_, Spt2Spec> {
        Cspt6W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Selection sample time of channel ADC_IN7"]
    #[inline(always)]
    pub fn cspt7(&mut self) -> Cspt7W<'_, Spt2Spec> {
        Cspt7W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Selection sample time of channel ADC_IN8"]
    #[inline(always)]
    pub fn cspt8(&mut self) -> Cspt8W<'_, Spt2Spec> {
        Cspt8W::new(self, 24)
    }
    #[doc = "Bits 27:29 - Selection sample time of channel ADC_IN9"]
    #[inline(always)]
    pub fn cspt9(&mut self) -> Cspt9W<'_, Spt2Spec> {
        Cspt9W::new(self, 27)
    }
}
#[doc = "sample time register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`spt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spt2Spec;
impl crate::RegisterSpec for Spt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spt2::R`](R) reader structure"]
impl crate::Readable for Spt2Spec {}
#[doc = "`write(|w| ..)` method takes [`spt2::W`](W) writer structure"]
impl crate::Writable for Spt2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPT2 to value 0"]
impl crate::Resettable for Spt2Spec {}
