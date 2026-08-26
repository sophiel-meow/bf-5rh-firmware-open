#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Field `VREXLPEN` reader - Voltage regulator extra low power mode enable"]
pub type VrexlpenR = crate::BitReader;
#[doc = "Field `VREXLPEN` writer - Voltage regulator extra low power mode enable"]
pub type VrexlpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Voltage regulator extra low power mode enable"]
    #[inline(always)]
    pub fn vrexlpen(&self) -> VrexlpenR {
        VrexlpenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Voltage regulator extra low power mode enable"]
    #[inline(always)]
    pub fn vrexlpen(&mut self) -> VrexlpenW<'_, Ctrl2Spec> {
        VrexlpenW::new(self, 5)
    }
}
#[doc = "Power control and status register2 (PWC_CTRL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl2Spec;
impl crate::RegisterSpec for Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for Ctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for Ctrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for Ctrl2Spec {}
