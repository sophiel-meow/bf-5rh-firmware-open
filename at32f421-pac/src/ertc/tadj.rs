#[doc = "Register `TADJ` writer"]
pub type W = crate::W<TadjSpec>;
#[doc = "Field `DECSBS` writer - Decrease sub-second value"]
pub type DecsbsW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `ADD1S` writer - Add 1 second"]
pub type Add1sW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:14 - Decrease sub-second value"]
    #[inline(always)]
    pub fn decsbs(&mut self) -> DecsbsW<'_, TadjSpec> {
        DecsbsW::new(self, 0)
    }
    #[doc = "Bit 31 - Add 1 second"]
    #[inline(always)]
    pub fn add1s(&mut self) -> Add1sW<'_, TadjSpec> {
        Add1sW::new(self, 31)
    }
}
#[doc = "time adjust register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tadj::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TadjSpec;
impl crate::RegisterSpec for TadjSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tadj::W`](W) writer structure"]
impl crate::Writable for TadjSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TADJ to value 0"]
impl crate::Resettable for TadjSpec {}
