#[doc = "Register `CRC_CTRL` reader"]
pub type R = crate::R<CrcCtrlSpec>;
#[doc = "Register `CRC_CTRL` writer"]
pub type W = crate::W<CrcCtrlSpec>;
#[doc = "Field `CRC_SN` reader - CRC sector numbler"]
pub type CrcSnR = crate::FieldReader<u16>;
#[doc = "Field `CRC_SN` writer - CRC sector numbler"]
pub type CrcSnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CRC_STRT` writer - CRC start"]
pub type CrcStrtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - CRC sector numbler"]
    #[inline(always)]
    pub fn crc_sn(&self) -> CrcSnR {
        CrcSnR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC sector numbler"]
    #[inline(always)]
    pub fn crc_sn(&mut self) -> CrcSnW<'_, CrcCtrlSpec> {
        CrcSnW::new(self, 0)
    }
    #[doc = "Bit 16 - CRC start"]
    #[inline(always)]
    pub fn crc_strt(&mut self) -> CrcStrtW<'_, CrcCtrlSpec> {
        CrcStrtW::new(self, 16)
    }
}
#[doc = "Flash CRC controll register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcCtrlSpec;
impl crate::RegisterSpec for CrcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_ctrl::R`](R) reader structure"]
impl crate::Readable for CrcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_ctrl::W`](W) writer structure"]
impl crate::Writable for CrcCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRC_CTRL to value 0"]
impl crate::Resettable for CrcCtrlSpec {}
