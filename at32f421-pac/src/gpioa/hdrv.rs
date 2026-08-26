#[doc = "Register `HDRV` reader"]
pub type R = crate::R<HdrvSpec>;
#[doc = "Register `HDRV` writer"]
pub type W = crate::W<HdrvSpec>;
#[doc = "Field `HDRV0` reader - Port x driver bit y"]
pub type Hdrv0R = crate::BitReader;
#[doc = "Field `HDRV0` writer - Port x driver bit y"]
pub type Hdrv0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDRV1` reader - Port x driver bit y"]
pub type Hdrv1R = crate::BitReader;
#[doc = "Field `HDRV1` writer - Port x driver bit y"]
pub type Hdrv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDRV2` reader - Port x driver bit y"]
pub type Hdrv2R = crate::BitReader;
#[doc = "Field `HDRV2` writer - Port x driver bit y"]
pub type Hdrv2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDRV3` reader - Port x driver bit y"]
pub type Hdrv3R = crate::BitReader;
#[doc = "Field `HDRV3` writer - Port x driver bit y"]
pub type Hdrv3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDRV4` reader - Port x driver bit y"]
pub type Hdrv4R = crate::BitReader;
#[doc = "Field `HDRV4` writer - Port x driver bit y"]
pub type Hdrv4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDRV5` reader - Port x driver bit y"]
pub type Hdrv5R = crate::BitReader;
#[doc = "Field `HDRV5` writer - Port x driver bit y"]
pub type Hdrv5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDRV6` reader - Port x driver bit y"]
pub type Hdrv6R = crate::BitReader;
#[doc = "Field `HDRV6` writer - Port x driver bit y"]
pub type Hdrv6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDRV7` reader - Port x driver bit y"]
pub type Hdrv7R = crate::BitReader;
#[doc = "Field `HDRV7` writer - Port x driver bit y"]
pub type Hdrv7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDRV8` reader - Port x driver bit y"]
pub type Hdrv8R = crate::BitReader;
#[doc = "Field `HDRV8` writer - Port x driver bit y"]
pub type Hdrv8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDRV9` reader - Port x driver bit y"]
pub type Hdrv9R = crate::BitReader;
#[doc = "Field `HDRV9` writer - Port x driver bit y"]
pub type Hdrv9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDRV10` reader - Port x driver bit y"]
pub type Hdrv10R = crate::BitReader;
#[doc = "Field `HDRV10` writer - Port x driver bit y"]
pub type Hdrv10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDRV11` reader - Port x driver bit y"]
pub type Hdrv11R = crate::BitReader;
#[doc = "Field `HDRV11` writer - Port x driver bit y"]
pub type Hdrv11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDRV12` reader - Port x driver bit y"]
pub type Hdrv12R = crate::BitReader;
#[doc = "Field `HDRV12` writer - Port x driver bit y"]
pub type Hdrv12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDRV13` reader - Port x driver bit y"]
pub type Hdrv13R = crate::BitReader;
#[doc = "Field `HDRV13` writer - Port x driver bit y"]
pub type Hdrv13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDRV14` reader - Port x driver bit y"]
pub type Hdrv14R = crate::BitReader;
#[doc = "Field `HDRV14` writer - Port x driver bit y"]
pub type Hdrv14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDRV15` reader - Port x driver bit y"]
pub type Hdrv15R = crate::BitReader;
#[doc = "Field `HDRV15` writer - Port x driver bit y"]
pub type Hdrv15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv0(&self) -> Hdrv0R {
        Hdrv0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv1(&self) -> Hdrv1R {
        Hdrv1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv2(&self) -> Hdrv2R {
        Hdrv2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv3(&self) -> Hdrv3R {
        Hdrv3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv4(&self) -> Hdrv4R {
        Hdrv4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv5(&self) -> Hdrv5R {
        Hdrv5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv6(&self) -> Hdrv6R {
        Hdrv6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv7(&self) -> Hdrv7R {
        Hdrv7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv8(&self) -> Hdrv8R {
        Hdrv8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv9(&self) -> Hdrv9R {
        Hdrv9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv10(&self) -> Hdrv10R {
        Hdrv10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv11(&self) -> Hdrv11R {
        Hdrv11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv12(&self) -> Hdrv12R {
        Hdrv12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv13(&self) -> Hdrv13R {
        Hdrv13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv14(&self) -> Hdrv14R {
        Hdrv14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv15(&self) -> Hdrv15R {
        Hdrv15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv0(&mut self) -> Hdrv0W<'_, HdrvSpec> {
        Hdrv0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv1(&mut self) -> Hdrv1W<'_, HdrvSpec> {
        Hdrv1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv2(&mut self) -> Hdrv2W<'_, HdrvSpec> {
        Hdrv2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv3(&mut self) -> Hdrv3W<'_, HdrvSpec> {
        Hdrv3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv4(&mut self) -> Hdrv4W<'_, HdrvSpec> {
        Hdrv4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv5(&mut self) -> Hdrv5W<'_, HdrvSpec> {
        Hdrv5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv6(&mut self) -> Hdrv6W<'_, HdrvSpec> {
        Hdrv6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv7(&mut self) -> Hdrv7W<'_, HdrvSpec> {
        Hdrv7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv8(&mut self) -> Hdrv8W<'_, HdrvSpec> {
        Hdrv8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv9(&mut self) -> Hdrv9W<'_, HdrvSpec> {
        Hdrv9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv10(&mut self) -> Hdrv10W<'_, HdrvSpec> {
        Hdrv10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv11(&mut self) -> Hdrv11W<'_, HdrvSpec> {
        Hdrv11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv12(&mut self) -> Hdrv12W<'_, HdrvSpec> {
        Hdrv12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv13(&mut self) -> Hdrv13W<'_, HdrvSpec> {
        Hdrv13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv14(&mut self) -> Hdrv14W<'_, HdrvSpec> {
        Hdrv14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port x driver bit y"]
    #[inline(always)]
    pub fn hdrv15(&mut self) -> Hdrv15W<'_, HdrvSpec> {
        Hdrv15W::new(self, 15)
    }
}
#[doc = "Huge current driver\n\nYou can [`read`](crate::Reg::read) this register and get [`hdrv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdrv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdrvSpec;
impl crate::RegisterSpec for HdrvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdrv::R`](R) reader structure"]
impl crate::Readable for HdrvSpec {}
#[doc = "`write(|w| ..)` method takes [`hdrv::W`](W) writer structure"]
impl crate::Writable for HdrvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HDRV to value 0"]
impl crate::Resettable for HdrvSpec {}
