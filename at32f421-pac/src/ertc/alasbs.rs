#[doc = "Register `ALASBS` reader"]
pub type R = crate::R<AlasbsSpec>;
#[doc = "Register `ALASBS` writer"]
pub type W = crate::W<AlasbsSpec>;
#[doc = "Field `SBS` reader - Sub-seconds value"]
pub type SbsR = crate::FieldReader<u16>;
#[doc = "Field `SBS` writer - Sub-seconds value"]
pub type SbsW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `SBSMSK` reader - Sub-second mask"]
pub type SbsmskR = crate::FieldReader;
#[doc = "Field `SBSMSK` writer - Sub-second mask"]
pub type SbsmskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:14 - Sub-seconds value"]
    #[inline(always)]
    pub fn sbs(&self) -> SbsR {
        SbsR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - Sub-second mask"]
    #[inline(always)]
    pub fn sbsmsk(&self) -> SbsmskR {
        SbsmskR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Sub-seconds value"]
    #[inline(always)]
    pub fn sbs(&mut self) -> SbsW<'_, AlasbsSpec> {
        SbsW::new(self, 0)
    }
    #[doc = "Bits 24:27 - Sub-second mask"]
    #[inline(always)]
    pub fn sbsmsk(&mut self) -> SbsmskW<'_, AlasbsSpec> {
        SbsmskW::new(self, 24)
    }
}
#[doc = "alarm A sub second register\n\nYou can [`read`](crate::Reg::read) this register and get [`alasbs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alasbs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlasbsSpec;
impl crate::RegisterSpec for AlasbsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alasbs::R`](R) reader structure"]
impl crate::Readable for AlasbsSpec {}
#[doc = "`write(|w| ..)` method takes [`alasbs::W`](W) writer structure"]
impl crate::Writable for AlasbsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALASBS to value 0"]
impl crate::Resettable for AlasbsSpec {}
