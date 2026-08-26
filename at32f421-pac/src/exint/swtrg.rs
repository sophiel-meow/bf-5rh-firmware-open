#[doc = "Register `SWTRG` reader"]
pub type R = crate::R<SwtrgSpec>;
#[doc = "Register `SWTRG` writer"]
pub type W = crate::W<SwtrgSpec>;
#[doc = "Field `SWT0` reader - Software triggle on line 0"]
pub type Swt0R = crate::BitReader;
#[doc = "Field `SWT0` writer - Software triggle on line 0"]
pub type Swt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWT1` reader - Software triggle on line 1"]
pub type Swt1R = crate::BitReader;
#[doc = "Field `SWT1` writer - Software triggle on line 1"]
pub type Swt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWT2` reader - Software triggle on line 2"]
pub type Swt2R = crate::BitReader;
#[doc = "Field `SWT2` writer - Software triggle on line 2"]
pub type Swt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWT3` reader - Software triggle on line 3"]
pub type Swt3R = crate::BitReader;
#[doc = "Field `SWT3` writer - Software triggle on line 3"]
pub type Swt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWT4` reader - Software triggle on line 4"]
pub type Swt4R = crate::BitReader;
#[doc = "Field `SWT4` writer - Software triggle on line 4"]
pub type Swt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWT5` reader - Software triggle on line 5"]
pub type Swt5R = crate::BitReader;
#[doc = "Field `SWT5` writer - Software triggle on line 5"]
pub type Swt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWT6` reader - Software triggle on line 6"]
pub type Swt6R = crate::BitReader;
#[doc = "Field `SWT6` writer - Software triggle on line 6"]
pub type Swt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWT7` reader - Software triggle on line 7"]
pub type Swt7R = crate::BitReader;
#[doc = "Field `SWT7` writer - Software triggle on line 7"]
pub type Swt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWT8` reader - Software triggle on line 8"]
pub type Swt8R = crate::BitReader;
#[doc = "Field `SWT8` writer - Software triggle on line 8"]
pub type Swt8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWT9` reader - Software triggle on line 9"]
pub type Swt9R = crate::BitReader;
#[doc = "Field `SWT9` writer - Software triggle on line 9"]
pub type Swt9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWT10` reader - Software triggle on line 10"]
pub type Swt10R = crate::BitReader;
#[doc = "Field `SWT10` writer - Software triggle on line 10"]
pub type Swt10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWT11` reader - Software triggle on line 11"]
pub type Swt11R = crate::BitReader;
#[doc = "Field `SWT11` writer - Software triggle on line 11"]
pub type Swt11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWT12` reader - Software triggle on line 12"]
pub type Swt12R = crate::BitReader;
#[doc = "Field `SWT12` writer - Software triggle on line 12"]
pub type Swt12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWT13` reader - Software triggle on line 13"]
pub type Swt13R = crate::BitReader;
#[doc = "Field `SWT13` writer - Software triggle on line 13"]
pub type Swt13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWT14` reader - Software triggle on line 14"]
pub type Swt14R = crate::BitReader;
#[doc = "Field `SWT14` writer - Software triggle on line 14"]
pub type Swt14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWT15` reader - Software triggle on line 15"]
pub type Swt15R = crate::BitReader;
#[doc = "Field `SWT15` writer - Software triggle on line 15"]
pub type Swt15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWT16` reader - Software triggle on line 16"]
pub type Swt16R = crate::BitReader;
#[doc = "Field `SWT16` writer - Software triggle on line 16"]
pub type Swt16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWT17` reader - Software triggle on line 17"]
pub type Swt17R = crate::BitReader;
#[doc = "Field `SWT17` writer - Software triggle on line 17"]
pub type Swt17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWT19` reader - Software triggle on line 19"]
pub type Swt19R = crate::BitReader;
#[doc = "Field `SWT19` writer - Software triggle on line 19"]
pub type Swt19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWT21` reader - Software triggle on line 21"]
pub type Swt21R = crate::BitReader;
#[doc = "Field `SWT21` writer - Software triggle on line 21"]
pub type Swt21W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software triggle on line 0"]
    #[inline(always)]
    pub fn swt0(&self) -> Swt0R {
        Swt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software triggle on line 1"]
    #[inline(always)]
    pub fn swt1(&self) -> Swt1R {
        Swt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software triggle on line 2"]
    #[inline(always)]
    pub fn swt2(&self) -> Swt2R {
        Swt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software triggle on line 3"]
    #[inline(always)]
    pub fn swt3(&self) -> Swt3R {
        Swt3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software triggle on line 4"]
    #[inline(always)]
    pub fn swt4(&self) -> Swt4R {
        Swt4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software triggle on line 5"]
    #[inline(always)]
    pub fn swt5(&self) -> Swt5R {
        Swt5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software triggle on line 6"]
    #[inline(always)]
    pub fn swt6(&self) -> Swt6R {
        Swt6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software triggle on line 7"]
    #[inline(always)]
    pub fn swt7(&self) -> Swt7R {
        Swt7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software triggle on line 8"]
    #[inline(always)]
    pub fn swt8(&self) -> Swt8R {
        Swt8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software triggle on line 9"]
    #[inline(always)]
    pub fn swt9(&self) -> Swt9R {
        Swt9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software triggle on line 10"]
    #[inline(always)]
    pub fn swt10(&self) -> Swt10R {
        Swt10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software triggle on line 11"]
    #[inline(always)]
    pub fn swt11(&self) -> Swt11R {
        Swt11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software triggle on line 12"]
    #[inline(always)]
    pub fn swt12(&self) -> Swt12R {
        Swt12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software triggle on line 13"]
    #[inline(always)]
    pub fn swt13(&self) -> Swt13R {
        Swt13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software triggle on line 14"]
    #[inline(always)]
    pub fn swt14(&self) -> Swt14R {
        Swt14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software triggle on line 15"]
    #[inline(always)]
    pub fn swt15(&self) -> Swt15R {
        Swt15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Software triggle on line 16"]
    #[inline(always)]
    pub fn swt16(&self) -> Swt16R {
        Swt16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Software triggle on line 17"]
    #[inline(always)]
    pub fn swt17(&self) -> Swt17R {
        Swt17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Software triggle on line 19"]
    #[inline(always)]
    pub fn swt19(&self) -> Swt19R {
        Swt19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Software triggle on line 21"]
    #[inline(always)]
    pub fn swt21(&self) -> Swt21R {
        Swt21R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software triggle on line 0"]
    #[inline(always)]
    pub fn swt0(&mut self) -> Swt0W<'_, SwtrgSpec> {
        Swt0W::new(self, 0)
    }
    #[doc = "Bit 1 - Software triggle on line 1"]
    #[inline(always)]
    pub fn swt1(&mut self) -> Swt1W<'_, SwtrgSpec> {
        Swt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Software triggle on line 2"]
    #[inline(always)]
    pub fn swt2(&mut self) -> Swt2W<'_, SwtrgSpec> {
        Swt2W::new(self, 2)
    }
    #[doc = "Bit 3 - Software triggle on line 3"]
    #[inline(always)]
    pub fn swt3(&mut self) -> Swt3W<'_, SwtrgSpec> {
        Swt3W::new(self, 3)
    }
    #[doc = "Bit 4 - Software triggle on line 4"]
    #[inline(always)]
    pub fn swt4(&mut self) -> Swt4W<'_, SwtrgSpec> {
        Swt4W::new(self, 4)
    }
    #[doc = "Bit 5 - Software triggle on line 5"]
    #[inline(always)]
    pub fn swt5(&mut self) -> Swt5W<'_, SwtrgSpec> {
        Swt5W::new(self, 5)
    }
    #[doc = "Bit 6 - Software triggle on line 6"]
    #[inline(always)]
    pub fn swt6(&mut self) -> Swt6W<'_, SwtrgSpec> {
        Swt6W::new(self, 6)
    }
    #[doc = "Bit 7 - Software triggle on line 7"]
    #[inline(always)]
    pub fn swt7(&mut self) -> Swt7W<'_, SwtrgSpec> {
        Swt7W::new(self, 7)
    }
    #[doc = "Bit 8 - Software triggle on line 8"]
    #[inline(always)]
    pub fn swt8(&mut self) -> Swt8W<'_, SwtrgSpec> {
        Swt8W::new(self, 8)
    }
    #[doc = "Bit 9 - Software triggle on line 9"]
    #[inline(always)]
    pub fn swt9(&mut self) -> Swt9W<'_, SwtrgSpec> {
        Swt9W::new(self, 9)
    }
    #[doc = "Bit 10 - Software triggle on line 10"]
    #[inline(always)]
    pub fn swt10(&mut self) -> Swt10W<'_, SwtrgSpec> {
        Swt10W::new(self, 10)
    }
    #[doc = "Bit 11 - Software triggle on line 11"]
    #[inline(always)]
    pub fn swt11(&mut self) -> Swt11W<'_, SwtrgSpec> {
        Swt11W::new(self, 11)
    }
    #[doc = "Bit 12 - Software triggle on line 12"]
    #[inline(always)]
    pub fn swt12(&mut self) -> Swt12W<'_, SwtrgSpec> {
        Swt12W::new(self, 12)
    }
    #[doc = "Bit 13 - Software triggle on line 13"]
    #[inline(always)]
    pub fn swt13(&mut self) -> Swt13W<'_, SwtrgSpec> {
        Swt13W::new(self, 13)
    }
    #[doc = "Bit 14 - Software triggle on line 14"]
    #[inline(always)]
    pub fn swt14(&mut self) -> Swt14W<'_, SwtrgSpec> {
        Swt14W::new(self, 14)
    }
    #[doc = "Bit 15 - Software triggle on line 15"]
    #[inline(always)]
    pub fn swt15(&mut self) -> Swt15W<'_, SwtrgSpec> {
        Swt15W::new(self, 15)
    }
    #[doc = "Bit 16 - Software triggle on line 16"]
    #[inline(always)]
    pub fn swt16(&mut self) -> Swt16W<'_, SwtrgSpec> {
        Swt16W::new(self, 16)
    }
    #[doc = "Bit 17 - Software triggle on line 17"]
    #[inline(always)]
    pub fn swt17(&mut self) -> Swt17W<'_, SwtrgSpec> {
        Swt17W::new(self, 17)
    }
    #[doc = "Bit 19 - Software triggle on line 19"]
    #[inline(always)]
    pub fn swt19(&mut self) -> Swt19W<'_, SwtrgSpec> {
        Swt19W::new(self, 19)
    }
    #[doc = "Bit 21 - Software triggle on line 21"]
    #[inline(always)]
    pub fn swt21(&mut self) -> Swt21W<'_, SwtrgSpec> {
        Swt21W::new(self, 21)
    }
}
#[doc = "Software triggle register (EXTINT_SWIE)\n\nYou can [`read`](crate::Reg::read) this register and get [`swtrg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwtrgSpec;
impl crate::RegisterSpec for SwtrgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swtrg::R`](R) reader structure"]
impl crate::Readable for SwtrgSpec {}
#[doc = "`write(|w| ..)` method takes [`swtrg::W`](W) writer structure"]
impl crate::Writable for SwtrgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWTRG to value 0"]
impl crate::Resettable for SwtrgSpec {}
