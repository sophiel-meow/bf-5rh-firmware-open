#[doc = "Register `CM2_INPUT` reader"]
pub type R = crate::R<Cm2InputSpec>;
#[doc = "Register `CM2_INPUT` writer"]
pub type W = crate::W<Cm2InputSpec>;
#[doc = "Field `C3C` reader - Channel 3 configure"]
pub type C3cR = crate::FieldReader;
#[doc = "Field `C3C` writer - Channel 3 configure"]
pub type C3cW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C3IDIV` reader - Channel 3 input divider"]
pub type C3idivR = crate::FieldReader;
#[doc = "Field `C3IDIV` writer - Channel 3 input divider"]
pub type C3idivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C3DF` reader - Channel 3 digital filter"]
pub type C3dfR = crate::FieldReader;
#[doc = "Field `C3DF` writer - Channel 3 digital filter"]
pub type C3dfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `C4C` reader - Channel 4 configure"]
pub type C4cR = crate::FieldReader;
#[doc = "Field `C4C` writer - Channel 4 configure"]
pub type C4cW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C4IDIV` reader - Channel 4 input divider"]
pub type C4idivR = crate::FieldReader;
#[doc = "Field `C4IDIV` writer - Channel 4 input divider"]
pub type C4idivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C4DF` reader - Channel 4 digital filter"]
pub type C4dfR = crate::FieldReader;
#[doc = "Field `C4DF` writer - Channel 4 digital filter"]
pub type C4dfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Channel 3 configure"]
    #[inline(always)]
    pub fn c3c(&self) -> C3cR {
        C3cR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 3 input divider"]
    #[inline(always)]
    pub fn c3idiv(&self) -> C3idivR {
        C3idivR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Channel 3 digital filter"]
    #[inline(always)]
    pub fn c3df(&self) -> C3dfR {
        C3dfR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Channel 4 configure"]
    #[inline(always)]
    pub fn c4c(&self) -> C4cR {
        C4cR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Channel 4 input divider"]
    #[inline(always)]
    pub fn c4idiv(&self) -> C4idivR {
        C4idivR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Channel 4 digital filter"]
    #[inline(always)]
    pub fn c4df(&self) -> C4dfR {
        C4dfR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 3 configure"]
    #[inline(always)]
    pub fn c3c(&mut self) -> C3cW<'_, Cm2InputSpec> {
        C3cW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Channel 3 input divider"]
    #[inline(always)]
    pub fn c3idiv(&mut self) -> C3idivW<'_, Cm2InputSpec> {
        C3idivW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Channel 3 digital filter"]
    #[inline(always)]
    pub fn c3df(&mut self) -> C3dfW<'_, Cm2InputSpec> {
        C3dfW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Channel 4 configure"]
    #[inline(always)]
    pub fn c4c(&mut self) -> C4cW<'_, Cm2InputSpec> {
        C4cW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Channel 4 input divider"]
    #[inline(always)]
    pub fn c4idiv(&mut self) -> C4idivW<'_, Cm2InputSpec> {
        C4idivW::new(self, 10)
    }
    #[doc = "Bits 12:15 - Channel 4 digital filter"]
    #[inline(always)]
    pub fn c4df(&mut self) -> C4dfW<'_, Cm2InputSpec> {
        C4dfW::new(self, 12)
    }
}
#[doc = "Channel input mode register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cm2_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm2_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm2InputSpec;
impl crate::RegisterSpec for Cm2InputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm2_input::R`](R) reader structure"]
impl crate::Readable for Cm2InputSpec {}
#[doc = "`write(|w| ..)` method takes [`cm2_input::W`](W) writer structure"]
impl crate::Writable for Cm2InputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CM2_INPUT to value 0"]
impl crate::Resettable for Cm2InputSpec {}
