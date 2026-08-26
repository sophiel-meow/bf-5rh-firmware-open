#[doc = "Register `RLD` reader"]
pub type R = crate::R<RldSpec>;
#[doc = "Register `RLD` writer"]
pub type W = crate::W<RldSpec>;
#[doc = "Field `RLD` reader - Reload value"]
pub type RldR = crate::FieldReader<u16>;
#[doc = "Field `RLD` writer - Reload value"]
pub type RldW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Reload value"]
    #[inline(always)]
    pub fn rld(&self) -> RldR {
        RldR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Reload value"]
    #[inline(always)]
    pub fn rld(&mut self) -> RldW<'_, RldSpec> {
        RldW::new(self, 0)
    }
}
#[doc = "Reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`rld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RldSpec;
impl crate::RegisterSpec for RldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rld::R`](R) reader structure"]
impl crate::Readable for RldSpec {}
#[doc = "`write(|w| ..)` method takes [`rld::W`](W) writer structure"]
impl crate::Writable for RldSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RLD to value 0x0fff"]
impl crate::Resettable for RldSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
