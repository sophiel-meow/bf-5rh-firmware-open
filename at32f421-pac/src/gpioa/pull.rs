#[doc = "Register `PULL` reader"]
pub type R = crate::R<PullSpec>;
#[doc = "Register `PULL` writer"]
pub type W = crate::W<PullSpec>;
#[doc = "Field `PULL0` reader - GPIOx pin 0 pull configuration"]
pub type Pull0R = crate::FieldReader;
#[doc = "Field `PULL0` writer - GPIOx pin 0 pull configuration"]
pub type Pull0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PULL1` reader - GPIOx pin 1 pull configuration"]
pub type Pull1R = crate::FieldReader;
#[doc = "Field `PULL1` writer - GPIOx pin 1 pull configuration"]
pub type Pull1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PULL2` reader - GPIOx pin 2 pull configuration"]
pub type Pull2R = crate::FieldReader;
#[doc = "Field `PULL2` writer - GPIOx pin 2 pull configuration"]
pub type Pull2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PULL3` reader - GPIOx pin 3 pull configuration"]
pub type Pull3R = crate::FieldReader;
#[doc = "Field `PULL3` writer - GPIOx pin 3 pull configuration"]
pub type Pull3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PULL4` reader - GPIOx pin 4 pull configuration"]
pub type Pull4R = crate::FieldReader;
#[doc = "Field `PULL4` writer - GPIOx pin 4 pull configuration"]
pub type Pull4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PULL5` reader - GPIOx pin 5 pull configuration"]
pub type Pull5R = crate::FieldReader;
#[doc = "Field `PULL5` writer - GPIOx pin 5 pull configuration"]
pub type Pull5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PULL6` reader - GPIOx pin 6 pull configuration"]
pub type Pull6R = crate::FieldReader;
#[doc = "Field `PULL6` writer - GPIOx pin 6 pull configuration"]
pub type Pull6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PULL7` reader - GPIOx pin 7 pull configuration"]
pub type Pull7R = crate::FieldReader;
#[doc = "Field `PULL7` writer - GPIOx pin 7 pull configuration"]
pub type Pull7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PULL8` reader - GPIOx pin 8 pull configuration"]
pub type Pull8R = crate::FieldReader;
#[doc = "Field `PULL8` writer - GPIOx pin 8 pull configuration"]
pub type Pull8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PULL9` reader - GPIOx pin 9 pull configuration"]
pub type Pull9R = crate::FieldReader;
#[doc = "Field `PULL9` writer - GPIOx pin 9 pull configuration"]
pub type Pull9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PULL10` reader - GPIOx pin 10 pull configuration"]
pub type Pull10R = crate::FieldReader;
#[doc = "Field `PULL10` writer - GPIOx pin 10 pull configuration"]
pub type Pull10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PULL11` reader - GPIOx pin 11 pull configuration"]
pub type Pull11R = crate::FieldReader;
#[doc = "Field `PULL11` writer - GPIOx pin 11 pull configuration"]
pub type Pull11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PULL12` reader - GPIOx pin 12 pull configuration"]
pub type Pull12R = crate::FieldReader;
#[doc = "Field `PULL12` writer - GPIOx pin 12 pull configuration"]
pub type Pull12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PULL13` reader - GPIOx pin 13 pull configuration"]
pub type Pull13R = crate::FieldReader;
#[doc = "Field `PULL13` writer - GPIOx pin 13 pull configuration"]
pub type Pull13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PULL14` reader - GPIOx pin 14 pull configuration"]
pub type Pull14R = crate::FieldReader;
#[doc = "Field `PULL14` writer - GPIOx pin 14 pull configuration"]
pub type Pull14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PULL15` reader - GPIOx pin 15 pull configuration"]
pub type Pull15R = crate::FieldReader;
#[doc = "Field `PULL15` writer - GPIOx pin 15 pull configuration"]
pub type Pull15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - GPIOx pin 0 pull configuration"]
    #[inline(always)]
    pub fn pull0(&self) -> Pull0R {
        Pull0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIOx pin 1 pull configuration"]
    #[inline(always)]
    pub fn pull1(&self) -> Pull1R {
        Pull1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIOx pin 2 pull configuration"]
    #[inline(always)]
    pub fn pull2(&self) -> Pull2R {
        Pull2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIOx pin 3 pull configuration"]
    #[inline(always)]
    pub fn pull3(&self) -> Pull3R {
        Pull3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIOx pin 4 pull configuration"]
    #[inline(always)]
    pub fn pull4(&self) -> Pull4R {
        Pull4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIOx pin 5 pull configuration"]
    #[inline(always)]
    pub fn pull5(&self) -> Pull5R {
        Pull5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIOx pin 6 pull configuration"]
    #[inline(always)]
    pub fn pull6(&self) -> Pull6R {
        Pull6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIOx pin 7 pull configuration"]
    #[inline(always)]
    pub fn pull7(&self) -> Pull7R {
        Pull7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - GPIOx pin 8 pull configuration"]
    #[inline(always)]
    pub fn pull8(&self) -> Pull8R {
        Pull8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - GPIOx pin 9 pull configuration"]
    #[inline(always)]
    pub fn pull9(&self) -> Pull9R {
        Pull9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GPIOx pin 10 pull configuration"]
    #[inline(always)]
    pub fn pull10(&self) -> Pull10R {
        Pull10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - GPIOx pin 11 pull configuration"]
    #[inline(always)]
    pub fn pull11(&self) -> Pull11R {
        Pull11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - GPIOx pin 12 pull configuration"]
    #[inline(always)]
    pub fn pull12(&self) -> Pull12R {
        Pull12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - GPIOx pin 13 pull configuration"]
    #[inline(always)]
    pub fn pull13(&self) -> Pull13R {
        Pull13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - GPIOx pin 14 pull configuration"]
    #[inline(always)]
    pub fn pull14(&self) -> Pull14R {
        Pull14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - GPIOx pin 15 pull configuration"]
    #[inline(always)]
    pub fn pull15(&self) -> Pull15R {
        Pull15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIOx pin 0 pull configuration"]
    #[inline(always)]
    pub fn pull0(&mut self) -> Pull0W<'_, PullSpec> {
        Pull0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIOx pin 1 pull configuration"]
    #[inline(always)]
    pub fn pull1(&mut self) -> Pull1W<'_, PullSpec> {
        Pull1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIOx pin 2 pull configuration"]
    #[inline(always)]
    pub fn pull2(&mut self) -> Pull2W<'_, PullSpec> {
        Pull2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIOx pin 3 pull configuration"]
    #[inline(always)]
    pub fn pull3(&mut self) -> Pull3W<'_, PullSpec> {
        Pull3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIOx pin 4 pull configuration"]
    #[inline(always)]
    pub fn pull4(&mut self) -> Pull4W<'_, PullSpec> {
        Pull4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIOx pin 5 pull configuration"]
    #[inline(always)]
    pub fn pull5(&mut self) -> Pull5W<'_, PullSpec> {
        Pull5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIOx pin 6 pull configuration"]
    #[inline(always)]
    pub fn pull6(&mut self) -> Pull6W<'_, PullSpec> {
        Pull6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIOx pin 7 pull configuration"]
    #[inline(always)]
    pub fn pull7(&mut self) -> Pull7W<'_, PullSpec> {
        Pull7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - GPIOx pin 8 pull configuration"]
    #[inline(always)]
    pub fn pull8(&mut self) -> Pull8W<'_, PullSpec> {
        Pull8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - GPIOx pin 9 pull configuration"]
    #[inline(always)]
    pub fn pull9(&mut self) -> Pull9W<'_, PullSpec> {
        Pull9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - GPIOx pin 10 pull configuration"]
    #[inline(always)]
    pub fn pull10(&mut self) -> Pull10W<'_, PullSpec> {
        Pull10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - GPIOx pin 11 pull configuration"]
    #[inline(always)]
    pub fn pull11(&mut self) -> Pull11W<'_, PullSpec> {
        Pull11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - GPIOx pin 12 pull configuration"]
    #[inline(always)]
    pub fn pull12(&mut self) -> Pull12W<'_, PullSpec> {
        Pull12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - GPIOx pin 13 pull configuration"]
    #[inline(always)]
    pub fn pull13(&mut self) -> Pull13W<'_, PullSpec> {
        Pull13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - GPIOx pin 14 pull configuration"]
    #[inline(always)]
    pub fn pull14(&mut self) -> Pull14W<'_, PullSpec> {
        Pull14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - GPIOx pin 15 pull configuration"]
    #[inline(always)]
    pub fn pull15(&mut self) -> Pull15W<'_, PullSpec> {
        Pull15W::new(self, 30)
    }
}
#[doc = "GPIO pull-up/pull-down register\n\nYou can [`read`](crate::Reg::read) this register and get [`pull::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pull::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PullSpec;
impl crate::RegisterSpec for PullSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pull::R`](R) reader structure"]
impl crate::Readable for PullSpec {}
#[doc = "`write(|w| ..)` method takes [`pull::W`](W) writer structure"]
impl crate::Writable for PullSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PULL to value 0"]
impl crate::Resettable for PullSpec {}
