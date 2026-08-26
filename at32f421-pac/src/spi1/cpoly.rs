#[doc = "Register `CPOLY` reader"]
pub type R = crate::R<CpolySpec>;
#[doc = "Register `CPOLY` writer"]
pub type W = crate::W<CpolySpec>;
#[doc = "Field `CPOLY` reader - CRC polynomial"]
pub type CpolyR = crate::FieldReader<u16>;
#[doc = "Field `CPOLY` writer - CRC polynomial"]
pub type CpolyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CRC polynomial"]
    #[inline(always)]
    pub fn cpoly(&self) -> CpolyR {
        CpolyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC polynomial"]
    #[inline(always)]
    pub fn cpoly(&mut self) -> CpolyW<'_, CpolySpec> {
        CpolyW::new(self, 0)
    }
}
#[doc = "CRC polynomial register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpoly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpoly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpolySpec;
impl crate::RegisterSpec for CpolySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpoly::R`](R) reader structure"]
impl crate::Readable for CpolySpec {}
#[doc = "`write(|w| ..)` method takes [`cpoly::W`](W) writer structure"]
impl crate::Writable for CpolySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPOLY to value 0x07"]
impl crate::Resettable for CpolySpec {
    const RESET_VALUE: u32 = 0x07;
}
