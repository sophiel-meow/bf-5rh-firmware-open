#[doc = "Register `CLR` reader"]
pub type R = crate::R<ClrSpec>;
#[doc = "Register `CLR` writer"]
pub type W = crate::W<ClrSpec>;
#[doc = "Field `GFC1` reader - Channel 1 Global flag clear"]
pub type Gfc1R = crate::BitReader;
#[doc = "Field `GFC1` writer - Channel 1 Global flag clear"]
pub type Gfc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTFC1` reader - Channel 1 full data transfer flag clear"]
pub type Fdtfc1R = crate::BitReader;
#[doc = "Field `FDTFC1` writer - Channel 1 full data transfer flag clear"]
pub type Fdtfc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDTFC1` reader - Channel 1 half data transfer flag clear"]
pub type Hdtfc1R = crate::BitReader;
#[doc = "Field `HDTFC1` writer - Channel 1 half data transfer flag clear"]
pub type Hdtfc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRFC1` reader - Channel 1 data transfer error flag clear"]
pub type Dterrfc1R = crate::BitReader;
#[doc = "Field `DTERRFC1` writer - Channel 1 data transfer error flag clear"]
pub type Dterrfc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFC2` reader - Channel 2 Global flag clear"]
pub type Gfc2R = crate::BitReader;
#[doc = "Field `GFC2` writer - Channel 2 Global flag clear"]
pub type Gfc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTFC2` reader - Channel 2 full data transfer flag clear"]
pub type Fdtfc2R = crate::BitReader;
#[doc = "Field `FDTFC2` writer - Channel 2 full data transfer flag clear"]
pub type Fdtfc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDTFC2` reader - Channel 2 half data transfer flag clear"]
pub type Hdtfc2R = crate::BitReader;
#[doc = "Field `HDTFC2` writer - Channel 2 half data transfer flag clear"]
pub type Hdtfc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRFC2` reader - Channel 2 data transfer error flag clear"]
pub type Dterrfc2R = crate::BitReader;
#[doc = "Field `DTERRFC2` writer - Channel 2 data transfer error flag clear"]
pub type Dterrfc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFC3` reader - Channel 3 Global flag clear"]
pub type Gfc3R = crate::BitReader;
#[doc = "Field `GFC3` writer - Channel 3 Global flag clear"]
pub type Gfc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTFC3` reader - Channel 3 full data transfer flag clear"]
pub type Fdtfc3R = crate::BitReader;
#[doc = "Field `FDTFC3` writer - Channel 3 full data transfer flag clear"]
pub type Fdtfc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDTFC3` reader - Channel 3 half data transfer flag clear"]
pub type Hdtfc3R = crate::BitReader;
#[doc = "Field `HDTFC3` writer - Channel 3 half data transfer flag clear"]
pub type Hdtfc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRFC3` reader - Channel 3 data transfer error flag clear"]
pub type Dterrfc3R = crate::BitReader;
#[doc = "Field `DTERRFC3` writer - Channel 3 data transfer error flag clear"]
pub type Dterrfc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFC4` reader - Channel 4 Global flag clear"]
pub type Gfc4R = crate::BitReader;
#[doc = "Field `GFC4` writer - Channel 4 Global flag clear"]
pub type Gfc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTFC4` reader - Channel 4 full data transfer flag clear"]
pub type Fdtfc4R = crate::BitReader;
#[doc = "Field `FDTFC4` writer - Channel 4 full data transfer flag clear"]
pub type Fdtfc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDTFC4` reader - Channel 4 half data transfer flag clear"]
pub type Hdtfc4R = crate::BitReader;
#[doc = "Field `HDTFC4` writer - Channel 4 half data transfer flag clear"]
pub type Hdtfc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRFC4` reader - Channel 4 data transfer error flag clear"]
pub type Dterrfc4R = crate::BitReader;
#[doc = "Field `DTERRFC4` writer - Channel 4 data transfer error flag clear"]
pub type Dterrfc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFC5` reader - Channel 5 Global flag clear"]
pub type Gfc5R = crate::BitReader;
#[doc = "Field `GFC5` writer - Channel 5 Global flag clear"]
pub type Gfc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTFC5` reader - Channel 5 full data transfer flag clear"]
pub type Fdtfc5R = crate::BitReader;
#[doc = "Field `FDTFC5` writer - Channel 5 full data transfer flag clear"]
pub type Fdtfc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDTFC5` reader - Channel 5 half data transfer flag clear"]
pub type Hdtfc5R = crate::BitReader;
#[doc = "Field `HDTFC5` writer - Channel 5 half data transfer flag clear"]
pub type Hdtfc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRFC5` reader - Channel 5 data transfer error flag clear"]
pub type Dterrfc5R = crate::BitReader;
#[doc = "Field `DTERRFC5` writer - Channel 5 data transfer error flag clear"]
pub type Dterrfc5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel 1 Global flag clear"]
    #[inline(always)]
    pub fn gfc1(&self) -> Gfc1R {
        Gfc1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 full data transfer flag clear"]
    #[inline(always)]
    pub fn fdtfc1(&self) -> Fdtfc1R {
        Fdtfc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 half data transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc1(&self) -> Hdtfc1R {
        Hdtfc1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 1 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc1(&self) -> Dterrfc1R {
        Dterrfc1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 2 Global flag clear"]
    #[inline(always)]
    pub fn gfc2(&self) -> Gfc2R {
        Gfc2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 2 full data transfer flag clear"]
    #[inline(always)]
    pub fn fdtfc2(&self) -> Fdtfc2R {
        Fdtfc2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 2 half data transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc2(&self) -> Hdtfc2R {
        Hdtfc2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 2 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc2(&self) -> Dterrfc2R {
        Dterrfc2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 3 Global flag clear"]
    #[inline(always)]
    pub fn gfc3(&self) -> Gfc3R {
        Gfc3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 3 full data transfer flag clear"]
    #[inline(always)]
    pub fn fdtfc3(&self) -> Fdtfc3R {
        Fdtfc3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 3 half data transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc3(&self) -> Hdtfc3R {
        Hdtfc3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc3(&self) -> Dterrfc3R {
        Dterrfc3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Global flag clear"]
    #[inline(always)]
    pub fn gfc4(&self) -> Gfc4R {
        Gfc4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 4 full data transfer flag clear"]
    #[inline(always)]
    pub fn fdtfc4(&self) -> Fdtfc4R {
        Fdtfc4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 4 half data transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc4(&self) -> Hdtfc4R {
        Hdtfc4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 4 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc4(&self) -> Dterrfc4R {
        Dterrfc4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 5 Global flag clear"]
    #[inline(always)]
    pub fn gfc5(&self) -> Gfc5R {
        Gfc5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 5 full data transfer flag clear"]
    #[inline(always)]
    pub fn fdtfc5(&self) -> Fdtfc5R {
        Fdtfc5R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 5 half data transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc5(&self) -> Hdtfc5R {
        Hdtfc5R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 5 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc5(&self) -> Dterrfc5R {
        Dterrfc5R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 1 Global flag clear"]
    #[inline(always)]
    pub fn gfc1(&mut self) -> Gfc1W<'_, ClrSpec> {
        Gfc1W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 full data transfer flag clear"]
    #[inline(always)]
    pub fn fdtfc1(&mut self) -> Fdtfc1W<'_, ClrSpec> {
        Fdtfc1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 1 half data transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc1(&mut self) -> Hdtfc1W<'_, ClrSpec> {
        Hdtfc1W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 1 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc1(&mut self) -> Dterrfc1W<'_, ClrSpec> {
        Dterrfc1W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 2 Global flag clear"]
    #[inline(always)]
    pub fn gfc2(&mut self) -> Gfc2W<'_, ClrSpec> {
        Gfc2W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 2 full data transfer flag clear"]
    #[inline(always)]
    pub fn fdtfc2(&mut self) -> Fdtfc2W<'_, ClrSpec> {
        Fdtfc2W::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 2 half data transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc2(&mut self) -> Hdtfc2W<'_, ClrSpec> {
        Hdtfc2W::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 2 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc2(&mut self) -> Dterrfc2W<'_, ClrSpec> {
        Dterrfc2W::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 3 Global flag clear"]
    #[inline(always)]
    pub fn gfc3(&mut self) -> Gfc3W<'_, ClrSpec> {
        Gfc3W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 3 full data transfer flag clear"]
    #[inline(always)]
    pub fn fdtfc3(&mut self) -> Fdtfc3W<'_, ClrSpec> {
        Fdtfc3W::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 3 half data transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc3(&mut self) -> Hdtfc3W<'_, ClrSpec> {
        Hdtfc3W::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 3 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc3(&mut self) -> Dterrfc3W<'_, ClrSpec> {
        Dterrfc3W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 4 Global flag clear"]
    #[inline(always)]
    pub fn gfc4(&mut self) -> Gfc4W<'_, ClrSpec> {
        Gfc4W::new(self, 12)
    }
    #[doc = "Bit 13 - Channel 4 full data transfer flag clear"]
    #[inline(always)]
    pub fn fdtfc4(&mut self) -> Fdtfc4W<'_, ClrSpec> {
        Fdtfc4W::new(self, 13)
    }
    #[doc = "Bit 14 - Channel 4 half data transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc4(&mut self) -> Hdtfc4W<'_, ClrSpec> {
        Hdtfc4W::new(self, 14)
    }
    #[doc = "Bit 15 - Channel 4 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc4(&mut self) -> Dterrfc4W<'_, ClrSpec> {
        Dterrfc4W::new(self, 15)
    }
    #[doc = "Bit 16 - Channel 5 Global flag clear"]
    #[inline(always)]
    pub fn gfc5(&mut self) -> Gfc5W<'_, ClrSpec> {
        Gfc5W::new(self, 16)
    }
    #[doc = "Bit 17 - Channel 5 full data transfer flag clear"]
    #[inline(always)]
    pub fn fdtfc5(&mut self) -> Fdtfc5W<'_, ClrSpec> {
        Fdtfc5W::new(self, 17)
    }
    #[doc = "Bit 18 - Channel 5 half data transfer flag clear"]
    #[inline(always)]
    pub fn hdtfc5(&mut self) -> Hdtfc5W<'_, ClrSpec> {
        Hdtfc5W::new(self, 18)
    }
    #[doc = "Bit 19 - Channel 5 data transfer error flag clear"]
    #[inline(always)]
    pub fn dterrfc5(&mut self) -> Dterrfc5W<'_, ClrSpec> {
        Dterrfc5W::new(self, 19)
    }
}
#[doc = "DMA flag clear register (DMA_CLR)\n\nYou can [`read`](crate::Reg::read) this register and get [`clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrSpec;
impl crate::RegisterSpec for ClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr::R`](R) reader structure"]
impl crate::Readable for ClrSpec {}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for ClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for ClrSpec {}
