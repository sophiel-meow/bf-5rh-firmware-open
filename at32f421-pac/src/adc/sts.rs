#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<StsSpec>;
#[doc = "Field `VMOR` reader - Voltage monitoring out of range flag"]
pub type VmorR = crate::BitReader;
#[doc = "Field `VMOR` writer - Voltage monitoring out of range flag"]
pub type VmorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCE` reader - Channels conversion end flag"]
pub type CceR = crate::BitReader;
#[doc = "Field `CCE` writer - Channels conversion end flag"]
pub type CceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCCE` reader - Preempted channels conversion end flag"]
pub type PcceR = crate::BitReader;
#[doc = "Field `PCCE` writer - Preempted channels conversion end flag"]
pub type PcceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCCS` reader - Preempted channel conversion start flag"]
pub type PccsR = crate::BitReader;
#[doc = "Field `PCCS` writer - Preempted channel conversion start flag"]
pub type PccsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCCS` reader - Ordinary channel conversion start flag"]
pub type OccsR = crate::BitReader;
#[doc = "Field `OCCS` writer - Ordinary channel conversion start flag"]
pub type OccsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Voltage monitoring out of range flag"]
    #[inline(always)]
    pub fn vmor(&self) -> VmorR {
        VmorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channels conversion end flag"]
    #[inline(always)]
    pub fn cce(&self) -> CceR {
        CceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Preempted channels conversion end flag"]
    #[inline(always)]
    pub fn pcce(&self) -> PcceR {
        PcceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Preempted channel conversion start flag"]
    #[inline(always)]
    pub fn pccs(&self) -> PccsR {
        PccsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ordinary channel conversion start flag"]
    #[inline(always)]
    pub fn occs(&self) -> OccsR {
        OccsR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage monitoring out of range flag"]
    #[inline(always)]
    pub fn vmor(&mut self) -> VmorW<'_, StsSpec> {
        VmorW::new(self, 0)
    }
    #[doc = "Bit 1 - Channels conversion end flag"]
    #[inline(always)]
    pub fn cce(&mut self) -> CceW<'_, StsSpec> {
        CceW::new(self, 1)
    }
    #[doc = "Bit 2 - Preempted channels conversion end flag"]
    #[inline(always)]
    pub fn pcce(&mut self) -> PcceW<'_, StsSpec> {
        PcceW::new(self, 2)
    }
    #[doc = "Bit 3 - Preempted channel conversion start flag"]
    #[inline(always)]
    pub fn pccs(&mut self) -> PccsW<'_, StsSpec> {
        PccsW::new(self, 3)
    }
    #[doc = "Bit 4 - Ordinary channel conversion start flag"]
    #[inline(always)]
    pub fn occs(&mut self) -> OccsW<'_, StsSpec> {
        OccsW::new(self, 4)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for StsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for StsSpec {}
