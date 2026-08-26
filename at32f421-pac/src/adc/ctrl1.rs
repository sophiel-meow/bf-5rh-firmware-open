#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `VMCSEL` reader - Voltage monitoring channel select"]
pub type VmcselR = crate::FieldReader;
#[doc = "Field `VMCSEL` writer - Voltage monitoring channel select"]
pub type VmcselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CCEIEN` reader - Channel conversion end interrupt enable"]
pub type CceienR = crate::BitReader;
#[doc = "Field `CCEIEN` writer - Channel conversion end interrupt enable"]
pub type CceienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMORIEN` reader - Voltage monitoring out of range interrupt enable"]
pub type VmorienR = crate::BitReader;
#[doc = "Field `VMORIEN` writer - Voltage monitoring out of range interrupt enable"]
pub type VmorienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCCEIEN` reader - Conversion end interrupt enable for preempted channels"]
pub type PcceienR = crate::BitReader;
#[doc = "Field `PCCEIEN` writer - Conversion end interrupt enable for preempted channels"]
pub type PcceienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SQEN` reader - Sequence mode enable"]
pub type SqenR = crate::BitReader;
#[doc = "Field `SQEN` writer - Sequence mode enable"]
pub type SqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMSGEN` reader - Voltage monitoring enable on a single channel"]
pub type VmsgenR = crate::BitReader;
#[doc = "Field `VMSGEN` writer - Voltage monitoring enable on a single channel"]
pub type VmsgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCAUTOEN` reader - Preempted group automatic conversion enable after ordinary group"]
pub type PcautoenR = crate::BitReader;
#[doc = "Field `PCAUTOEN` writer - Preempted group automatic conversion enable after ordinary group"]
pub type PcautoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCPEN` reader - Partitioned mode enable on ordinary channels"]
pub type OcpenR = crate::BitReader;
#[doc = "Field `OCPEN` writer - Partitioned mode enable on ordinary channels"]
pub type OcpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCPEN` reader - Partitioned mode enable on preempted channels"]
pub type PcpenR = crate::BitReader;
#[doc = "Field `PCPEN` writer - Partitioned mode enable on preempted channels"]
pub type PcpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCPCNT` reader - Partitioned mode conversion count of ordinary channels"]
pub type OcpcntR = crate::FieldReader;
#[doc = "Field `OCPCNT` writer - Partitioned mode conversion count of ordinary channels"]
pub type OcpcntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PCVMEN` reader - Voltage monitoring enable on preempted channels"]
pub type PcvmenR = crate::BitReader;
#[doc = "Field `PCVMEN` writer - Voltage monitoring enable on preempted channels"]
pub type PcvmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCVMEN` reader - Voltage monitoring enable on ordinary channels"]
pub type OcvmenR = crate::BitReader;
#[doc = "Field `OCVMEN` writer - Voltage monitoring enable on ordinary channels"]
pub type OcvmenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Voltage monitoring channel select"]
    #[inline(always)]
    pub fn vmcsel(&self) -> VmcselR {
        VmcselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Channel conversion end interrupt enable"]
    #[inline(always)]
    pub fn cceien(&self) -> CceienR {
        CceienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Voltage monitoring out of range interrupt enable"]
    #[inline(always)]
    pub fn vmorien(&self) -> VmorienR {
        VmorienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Conversion end interrupt enable for preempted channels"]
    #[inline(always)]
    pub fn pcceien(&self) -> PcceienR {
        PcceienR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Sequence mode enable"]
    #[inline(always)]
    pub fn sqen(&self) -> SqenR {
        SqenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Voltage monitoring enable on a single channel"]
    #[inline(always)]
    pub fn vmsgen(&self) -> VmsgenR {
        VmsgenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Preempted group automatic conversion enable after ordinary group"]
    #[inline(always)]
    pub fn pcautoen(&self) -> PcautoenR {
        PcautoenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Partitioned mode enable on ordinary channels"]
    #[inline(always)]
    pub fn ocpen(&self) -> OcpenR {
        OcpenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Partitioned mode enable on preempted channels"]
    #[inline(always)]
    pub fn pcpen(&self) -> PcpenR {
        PcpenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Partitioned mode conversion count of ordinary channels"]
    #[inline(always)]
    pub fn ocpcnt(&self) -> OcpcntR {
        OcpcntR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 22 - Voltage monitoring enable on preempted channels"]
    #[inline(always)]
    pub fn pcvmen(&self) -> PcvmenR {
        PcvmenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Voltage monitoring enable on ordinary channels"]
    #[inline(always)]
    pub fn ocvmen(&self) -> OcvmenR {
        OcvmenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Voltage monitoring channel select"]
    #[inline(always)]
    pub fn vmcsel(&mut self) -> VmcselW<'_, Ctrl1Spec> {
        VmcselW::new(self, 0)
    }
    #[doc = "Bit 5 - Channel conversion end interrupt enable"]
    #[inline(always)]
    pub fn cceien(&mut self) -> CceienW<'_, Ctrl1Spec> {
        CceienW::new(self, 5)
    }
    #[doc = "Bit 6 - Voltage monitoring out of range interrupt enable"]
    #[inline(always)]
    pub fn vmorien(&mut self) -> VmorienW<'_, Ctrl1Spec> {
        VmorienW::new(self, 6)
    }
    #[doc = "Bit 7 - Conversion end interrupt enable for preempted channels"]
    #[inline(always)]
    pub fn pcceien(&mut self) -> PcceienW<'_, Ctrl1Spec> {
        PcceienW::new(self, 7)
    }
    #[doc = "Bit 8 - Sequence mode enable"]
    #[inline(always)]
    pub fn sqen(&mut self) -> SqenW<'_, Ctrl1Spec> {
        SqenW::new(self, 8)
    }
    #[doc = "Bit 9 - Voltage monitoring enable on a single channel"]
    #[inline(always)]
    pub fn vmsgen(&mut self) -> VmsgenW<'_, Ctrl1Spec> {
        VmsgenW::new(self, 9)
    }
    #[doc = "Bit 10 - Preempted group automatic conversion enable after ordinary group"]
    #[inline(always)]
    pub fn pcautoen(&mut self) -> PcautoenW<'_, Ctrl1Spec> {
        PcautoenW::new(self, 10)
    }
    #[doc = "Bit 11 - Partitioned mode enable on ordinary channels"]
    #[inline(always)]
    pub fn ocpen(&mut self) -> OcpenW<'_, Ctrl1Spec> {
        OcpenW::new(self, 11)
    }
    #[doc = "Bit 12 - Partitioned mode enable on preempted channels"]
    #[inline(always)]
    pub fn pcpen(&mut self) -> PcpenW<'_, Ctrl1Spec> {
        PcpenW::new(self, 12)
    }
    #[doc = "Bits 13:15 - Partitioned mode conversion count of ordinary channels"]
    #[inline(always)]
    pub fn ocpcnt(&mut self) -> OcpcntW<'_, Ctrl1Spec> {
        OcpcntW::new(self, 13)
    }
    #[doc = "Bit 22 - Voltage monitoring enable on preempted channels"]
    #[inline(always)]
    pub fn pcvmen(&mut self) -> PcvmenW<'_, Ctrl1Spec> {
        PcvmenW::new(self, 22)
    }
    #[doc = "Bit 23 - Voltage monitoring enable on ordinary channels"]
    #[inline(always)]
    pub fn ocvmen(&mut self) -> OcvmenW<'_, Ctrl1Spec> {
        OcvmenW::new(self, 23)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for Ctrl1Spec {}
