#[doc = "Register `SPT1` reader"]
pub type R = crate::R<Spt1Spec>;
#[doc = "Register `SPT1` writer"]
pub type W = crate::W<Spt1Spec>;
#[doc = "Field `CSPT10` reader - Selection sample time of channel ADC_IN10"]
pub type Cspt10R = crate::FieldReader;
#[doc = "Field `CSPT10` writer - Selection sample time of channel ADC_IN10"]
pub type Cspt10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSPT11` reader - Selection sample time of channel ADC_IN11"]
pub type Cspt11R = crate::FieldReader;
#[doc = "Field `CSPT11` writer - Selection sample time of channel ADC_IN11"]
pub type Cspt11W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSPT12` reader - Selection sample time of channel ADC_IN12"]
pub type Cspt12R = crate::FieldReader;
#[doc = "Field `CSPT12` writer - Selection sample time of channel ADC_IN12"]
pub type Cspt12W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSPT13` reader - Selection sample time of channel ADC_IN13"]
pub type Cspt13R = crate::FieldReader;
#[doc = "Field `CSPT13` writer - Selection sample time of channel ADC_IN13"]
pub type Cspt13W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSPT14` reader - Selection sample time of channel ADC_IN14"]
pub type Cspt14R = crate::FieldReader;
#[doc = "Field `CSPT14` writer - Selection sample time of channel ADC_IN14"]
pub type Cspt14W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSPT15` reader - Selection sample time of channel ADC_IN15"]
pub type Cspt15R = crate::FieldReader;
#[doc = "Field `CSPT15` writer - Selection sample time of channel ADC_IN15"]
pub type Cspt15W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSPT16` reader - Selection sample time of channel ADC_IN16"]
pub type Cspt16R = crate::FieldReader;
#[doc = "Field `CSPT16` writer - Selection sample time of channel ADC_IN16"]
pub type Cspt16W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSPT17` reader - Selection sample time of channel ADC_IN17"]
pub type Cspt17R = crate::FieldReader;
#[doc = "Field `CSPT17` writer - Selection sample time of channel ADC_IN17"]
pub type Cspt17W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN10"]
    #[inline(always)]
    pub fn cspt10(&self) -> Cspt10R {
        Cspt10R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN11"]
    #[inline(always)]
    pub fn cspt11(&self) -> Cspt11R {
        Cspt11R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Selection sample time of channel ADC_IN12"]
    #[inline(always)]
    pub fn cspt12(&self) -> Cspt12R {
        Cspt12R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Selection sample time of channel ADC_IN13"]
    #[inline(always)]
    pub fn cspt13(&self) -> Cspt13R {
        Cspt13R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Selection sample time of channel ADC_IN14"]
    #[inline(always)]
    pub fn cspt14(&self) -> Cspt14R {
        Cspt14R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Selection sample time of channel ADC_IN15"]
    #[inline(always)]
    pub fn cspt15(&self) -> Cspt15R {
        Cspt15R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Selection sample time of channel ADC_IN16"]
    #[inline(always)]
    pub fn cspt16(&self) -> Cspt16R {
        Cspt16R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Selection sample time of channel ADC_IN17"]
    #[inline(always)]
    pub fn cspt17(&self) -> Cspt17R {
        Cspt17R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN10"]
    #[inline(always)]
    pub fn cspt10(&mut self) -> Cspt10W<'_, Spt1Spec> {
        Cspt10W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN11"]
    #[inline(always)]
    pub fn cspt11(&mut self) -> Cspt11W<'_, Spt1Spec> {
        Cspt11W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Selection sample time of channel ADC_IN12"]
    #[inline(always)]
    pub fn cspt12(&mut self) -> Cspt12W<'_, Spt1Spec> {
        Cspt12W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Selection sample time of channel ADC_IN13"]
    #[inline(always)]
    pub fn cspt13(&mut self) -> Cspt13W<'_, Spt1Spec> {
        Cspt13W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Selection sample time of channel ADC_IN14"]
    #[inline(always)]
    pub fn cspt14(&mut self) -> Cspt14W<'_, Spt1Spec> {
        Cspt14W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Selection sample time of channel ADC_IN15"]
    #[inline(always)]
    pub fn cspt15(&mut self) -> Cspt15W<'_, Spt1Spec> {
        Cspt15W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Selection sample time of channel ADC_IN16"]
    #[inline(always)]
    pub fn cspt16(&mut self) -> Cspt16W<'_, Spt1Spec> {
        Cspt16W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Selection sample time of channel ADC_IN17"]
    #[inline(always)]
    pub fn cspt17(&mut self) -> Cspt17W<'_, Spt1Spec> {
        Cspt17W::new(self, 21)
    }
}
#[doc = "sample time register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`spt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spt1Spec;
impl crate::RegisterSpec for Spt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spt1::R`](R) reader structure"]
impl crate::Readable for Spt1Spec {}
#[doc = "`write(|w| ..)` method takes [`spt1::W`](W) writer structure"]
impl crate::Writable for Spt1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPT1 to value 0"]
impl crate::Resettable for Spt1Spec {}
