#[doc = "Register `CVAL` reader"]
pub type R = crate::R<CvalSpec>;
#[doc = "Register `CVAL` writer"]
pub type W = crate::W<CvalSpec>;
#[doc = "Field `CVAL` reader - Counter value"]
pub type CvalR = crate::FieldReader<u16>;
#[doc = "Field `CVAL` writer - Counter value"]
pub type CvalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter value"]
    #[inline(always)]
    pub fn cval(&self) -> CvalR {
        CvalR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter value"]
    #[inline(always)]
    pub fn cval(&mut self) -> CvalW<'_, CvalSpec> {
        CvalW::new(self, 0)
    }
}
#[doc = "Counter value\n\nYou can [`read`](crate::Reg::read) this register and get [`cval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CvalSpec;
impl crate::RegisterSpec for CvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cval::R`](R) reader structure"]
impl crate::Readable for CvalSpec {}
#[doc = "`write(|w| ..)` method takes [`cval::W`](W) writer structure"]
impl crate::Writable for CvalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CVAL to value 0"]
impl crate::Resettable for CvalSpec {}
