#[doc = "Register `CRC_ADDR` writer"]
pub type W = crate::W<CrcAddrSpec>;
#[doc = "Field `CRC_ADDR` writer - CRC address"]
pub type CrcAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - CRC address"]
    #[inline(always)]
    pub fn crc_addr(&mut self) -> CrcAddrW<'_, CrcAddrSpec> {
        CrcAddrW::new(self, 0)
    }
}
#[doc = "Flash CRC data start address register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_addr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcAddrSpec;
impl crate::RegisterSpec for CrcAddrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crc_addr::W`](W) writer structure"]
impl crate::Writable for CrcAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRC_ADDR to value 0"]
impl crate::Resettable for CrcAddrSpec {}
