#[doc = "Register `CM1_INPUT` reader"]
pub type R = crate::R<Cm1InputSpec>;
#[doc = "Register `CM1_INPUT` writer"]
pub type W = crate::W<Cm1InputSpec>;
#[doc = "Field `C1C` reader - Channel 1 configure"]
pub type C1cR = crate::FieldReader;
#[doc = "Field `C1C` writer - Channel 1 configure"]
pub type C1cW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C1IDIV` reader - Channel 1 input divider"]
pub type C1idivR = crate::FieldReader;
#[doc = "Field `C1IDIV` writer - Channel 1 input divider"]
pub type C1idivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C1DF` reader - Channel 1 digital filter"]
pub type C1dfR = crate::FieldReader;
#[doc = "Field `C1DF` writer - Channel 1 digital filter"]
pub type C1dfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Channel 1 configure"]
    #[inline(always)]
    pub fn c1c(&self) -> C1cR {
        C1cR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 1 input divider"]
    #[inline(always)]
    pub fn c1idiv(&self) -> C1idivR {
        C1idivR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Channel 1 digital filter"]
    #[inline(always)]
    pub fn c1df(&self) -> C1dfR {
        C1dfR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 1 configure"]
    #[inline(always)]
    pub fn c1c(&mut self) -> C1cW<'_, Cm1InputSpec> {
        C1cW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Channel 1 input divider"]
    #[inline(always)]
    pub fn c1idiv(&mut self) -> C1idivW<'_, Cm1InputSpec> {
        C1idivW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Channel 1 digital filter"]
    #[inline(always)]
    pub fn c1df(&mut self) -> C1dfW<'_, Cm1InputSpec> {
        C1dfW::new(self, 4)
    }
}
#[doc = "Channel input mode register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cm1_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm1_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm1InputSpec;
impl crate::RegisterSpec for Cm1InputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm1_input::R`](R) reader structure"]
impl crate::Readable for Cm1InputSpec {}
#[doc = "`write(|w| ..)` method takes [`cm1_input::W`](W) writer structure"]
impl crate::Writable for Cm1InputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CM1_INPUT to value 0"]
impl crate::Resettable for Cm1InputSpec {}
