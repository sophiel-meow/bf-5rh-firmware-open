#[doc = "Register `VMLB` reader"]
pub type R = crate::R<VmlbSpec>;
#[doc = "Register `VMLB` writer"]
pub type W = crate::W<VmlbSpec>;
#[doc = "Field `VMLB` reader - Voltage monitoring low boundary"]
pub type VmlbR = crate::FieldReader<u16>;
#[doc = "Field `VMLB` writer - Voltage monitoring low boundary"]
pub type VmlbW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Voltage monitoring low boundary"]
    #[inline(always)]
    pub fn vmlb(&self) -> VmlbR {
        VmlbR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Voltage monitoring low boundary"]
    #[inline(always)]
    pub fn vmlb(&mut self) -> VmlbW<'_, VmlbSpec> {
        VmlbW::new(self, 0)
    }
}
#[doc = "Voltage monitoring low boundary register\n\nYou can [`read`](crate::Reg::read) this register and get [`vmlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VmlbSpec;
impl crate::RegisterSpec for VmlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmlb::R`](R) reader structure"]
impl crate::Readable for VmlbSpec {}
#[doc = "`write(|w| ..)` method takes [`vmlb::W`](W) writer structure"]
impl crate::Writable for VmlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VMLB to value 0"]
impl crate::Resettable for VmlbSpec {}
