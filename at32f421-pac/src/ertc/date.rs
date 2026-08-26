#[doc = "Register `DATE` reader"]
pub type R = crate::R<DateSpec>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DateSpec>;
#[doc = "Field `DU` reader - Date units"]
pub type DuR = crate::FieldReader;
#[doc = "Field `DU` writer - Date units"]
pub type DuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DT` reader - Date tens"]
pub type DtR = crate::FieldReader;
#[doc = "Field `DT` writer - Date tens"]
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MU` reader - Month units"]
pub type MuR = crate::FieldReader;
#[doc = "Field `MU` writer - Month units"]
pub type MuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MT` reader - Month tens"]
pub type MtR = crate::BitReader;
#[doc = "Field `MT` writer - Month tens"]
pub type MtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WK` reader - Week"]
pub type WkR = crate::FieldReader;
#[doc = "Field `WK` writer - Week"]
pub type WkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `YU` reader - Year units"]
pub type YuR = crate::FieldReader;
#[doc = "Field `YU` writer - Year units"]
pub type YuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YT` reader - Year tens"]
pub type YtR = crate::FieldReader;
#[doc = "Field `YT` writer - Year tens"]
pub type YtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Date units"]
    #[inline(always)]
    pub fn du(&self) -> DuR {
        DuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Date tens"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Month units"]
    #[inline(always)]
    pub fn mu(&self) -> MuR {
        MuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month tens"]
    #[inline(always)]
    pub fn mt(&self) -> MtR {
        MtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Week"]
    #[inline(always)]
    pub fn wk(&self) -> WkR {
        WkR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Year units"]
    #[inline(always)]
    pub fn yu(&self) -> YuR {
        YuR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Year tens"]
    #[inline(always)]
    pub fn yt(&self) -> YtR {
        YtR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Date units"]
    #[inline(always)]
    pub fn du(&mut self) -> DuW<'_, DateSpec> {
        DuW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Date tens"]
    #[inline(always)]
    pub fn dt(&mut self) -> DtW<'_, DateSpec> {
        DtW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Month units"]
    #[inline(always)]
    pub fn mu(&mut self) -> MuW<'_, DateSpec> {
        MuW::new(self, 8)
    }
    #[doc = "Bit 12 - Month tens"]
    #[inline(always)]
    pub fn mt(&mut self) -> MtW<'_, DateSpec> {
        MtW::new(self, 12)
    }
    #[doc = "Bits 13:15 - Week"]
    #[inline(always)]
    pub fn wk(&mut self) -> WkW<'_, DateSpec> {
        WkW::new(self, 13)
    }
    #[doc = "Bits 16:19 - Year units"]
    #[inline(always)]
    pub fn yu(&mut self) -> YuW<'_, DateSpec> {
        YuW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Year tens"]
    #[inline(always)]
    pub fn yt(&mut self) -> YtW<'_, DateSpec> {
        YtW::new(self, 20)
    }
}
#[doc = "date register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DateSpec;
impl crate::RegisterSpec for DateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DateSpec {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATE to value 0x2101"]
impl crate::Resettable for DateSpec {
    const RESET_VALUE: u32 = 0x2101;
}
