#[doc = "Register `BRK` reader"]
pub type R = crate::R<BrkSpec>;
#[doc = "Register `BRK` writer"]
pub type W = crate::W<BrkSpec>;
#[doc = "Field `DTC` reader - Dead-time configuration"]
pub type DtcR = crate::FieldReader;
#[doc = "Field `DTC` writer - Dead-time configuration"]
pub type DtcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WPC` reader - Write protected configuration"]
pub type WpcR = crate::FieldReader;
#[doc = "Field `WPC` writer - Write protected configuration"]
pub type WpcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FCSODIS` reader - Frozen channel status when holistic output disable"]
pub type FcsodisR = crate::BitReader;
#[doc = "Field `FCSODIS` writer - Frozen channel status when holistic output disable"]
pub type FcsodisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCSOEN` reader - Frozen channel status when holistic output enable"]
pub type FcsoenR = crate::BitReader;
#[doc = "Field `FCSOEN` writer - Frozen channel status when holistic output enable"]
pub type FcsoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRKEN` reader - Brake enable"]
pub type BrkenR = crate::BitReader;
#[doc = "Field `BRKEN` writer - Brake enable"]
pub type BrkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRKV` reader - Brake input validity"]
pub type BrkvR = crate::BitReader;
#[doc = "Field `BRKV` writer - Brake input validity"]
pub type BrkvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AOEN` reader - Automatic output enable"]
pub type AoenR = crate::BitReader;
#[doc = "Field `AOEN` writer - Automatic output enable"]
pub type AoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEN` reader - Output enable"]
pub type OenR = crate::BitReader;
#[doc = "Field `OEN` writer - Output enable"]
pub type OenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Dead-time configuration"]
    #[inline(always)]
    pub fn dtc(&self) -> DtcR {
        DtcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Write protected configuration"]
    #[inline(always)]
    pub fn wpc(&self) -> WpcR {
        WpcR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Frozen channel status when holistic output disable"]
    #[inline(always)]
    pub fn fcsodis(&self) -> FcsodisR {
        FcsodisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Frozen channel status when holistic output enable"]
    #[inline(always)]
    pub fn fcsoen(&self) -> FcsoenR {
        FcsoenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Brake enable"]
    #[inline(always)]
    pub fn brken(&self) -> BrkenR {
        BrkenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Brake input validity"]
    #[inline(always)]
    pub fn brkv(&self) -> BrkvR {
        BrkvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    pub fn aoen(&self) -> AoenR {
        AoenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Output enable"]
    #[inline(always)]
    pub fn oen(&self) -> OenR {
        OenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time configuration"]
    #[inline(always)]
    pub fn dtc(&mut self) -> DtcW<'_, BrkSpec> {
        DtcW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Write protected configuration"]
    #[inline(always)]
    pub fn wpc(&mut self) -> WpcW<'_, BrkSpec> {
        WpcW::new(self, 8)
    }
    #[doc = "Bit 10 - Frozen channel status when holistic output disable"]
    #[inline(always)]
    pub fn fcsodis(&mut self) -> FcsodisW<'_, BrkSpec> {
        FcsodisW::new(self, 10)
    }
    #[doc = "Bit 11 - Frozen channel status when holistic output enable"]
    #[inline(always)]
    pub fn fcsoen(&mut self) -> FcsoenW<'_, BrkSpec> {
        FcsoenW::new(self, 11)
    }
    #[doc = "Bit 12 - Brake enable"]
    #[inline(always)]
    pub fn brken(&mut self) -> BrkenW<'_, BrkSpec> {
        BrkenW::new(self, 12)
    }
    #[doc = "Bit 13 - Brake input validity"]
    #[inline(always)]
    pub fn brkv(&mut self) -> BrkvW<'_, BrkSpec> {
        BrkvW::new(self, 13)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    pub fn aoen(&mut self) -> AoenW<'_, BrkSpec> {
        AoenW::new(self, 14)
    }
    #[doc = "Bit 15 - Output enable"]
    #[inline(always)]
    pub fn oen(&mut self) -> OenW<'_, BrkSpec> {
        OenW::new(self, 15)
    }
}
#[doc = "Brake register\n\nYou can [`read`](crate::Reg::read) this register and get [`brk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrkSpec;
impl crate::RegisterSpec for BrkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brk::R`](R) reader structure"]
impl crate::Readable for BrkSpec {}
#[doc = "`write(|w| ..)` method takes [`brk::W`](W) writer structure"]
impl crate::Writable for BrkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRK to value 0"]
impl crate::Resettable for BrkSpec {}
