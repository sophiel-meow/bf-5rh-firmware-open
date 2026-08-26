#[doc = "Register `DIV` reader"]
pub type R = crate::R<DivSpec>;
#[doc = "Register `DIV` writer"]
pub type W = crate::W<DivSpec>;
#[doc = "Field `DIVB` reader - Diveder B"]
pub type DivbR = crate::FieldReader<u16>;
#[doc = "Field `DIVB` writer - Diveder B"]
pub type DivbW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `DIVA` reader - Diveder A"]
pub type DivaR = crate::FieldReader;
#[doc = "Field `DIVA` writer - Diveder A"]
pub type DivaW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:14 - Diveder B"]
    #[inline(always)]
    pub fn divb(&self) -> DivbR {
        DivbR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:22 - Diveder A"]
    #[inline(always)]
    pub fn diva(&self) -> DivaR {
        DivaR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Diveder B"]
    #[inline(always)]
    pub fn divb(&mut self) -> DivbW<'_, DivSpec> {
        DivbW::new(self, 0)
    }
    #[doc = "Bits 16:22 - Diveder A"]
    #[inline(always)]
    pub fn diva(&mut self) -> DivaW<'_, DivSpec> {
        DivaW::new(self, 16)
    }
}
#[doc = "Diveder register\n\nYou can [`read`](crate::Reg::read) this register and get [`div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivSpec;
impl crate::RegisterSpec for DivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div::R`](R) reader structure"]
impl crate::Readable for DivSpec {}
#[doc = "`write(|w| ..)` method takes [`div::W`](W) writer structure"]
impl crate::Writable for DivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIV to value 0x007f_00ff"]
impl crate::Resettable for DivSpec {
    const RESET_VALUE: u32 = 0x007f_00ff;
}
