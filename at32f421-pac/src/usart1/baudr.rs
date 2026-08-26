#[doc = "Register `BAUDR` reader"]
pub type R = crate::R<BaudrSpec>;
#[doc = "Register `BAUDR` writer"]
pub type W = crate::W<BaudrSpec>;
#[doc = "Field `DIV` reader - Division"]
pub type DivR = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Division"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Division"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Division"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, BaudrSpec> {
        DivW::new(self, 0)
    }
}
#[doc = "Baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`baudr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baudr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaudrSpec;
impl crate::RegisterSpec for BaudrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baudr::R`](R) reader structure"]
impl crate::Readable for BaudrSpec {}
#[doc = "`write(|w| ..)` method takes [`baudr::W`](W) writer structure"]
impl crate::Writable for BaudrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAUDR to value 0"]
impl crate::Resettable for BaudrSpec {}
