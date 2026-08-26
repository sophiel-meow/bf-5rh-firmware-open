#[doc = "Register `GDIV` reader"]
pub type R = crate::R<GdivSpec>;
#[doc = "Register `GDIV` writer"]
pub type W = crate::W<GdivSpec>;
#[doc = "Field `ISDIV` reader - IrDA/smartcard division value"]
pub type IsdivR = crate::FieldReader;
#[doc = "Field `ISDIV` writer - IrDA/smartcard division value"]
pub type IsdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCGT` reader - Smart card guard time value"]
pub type ScgtR = crate::FieldReader;
#[doc = "Field `SCGT` writer - Smart card guard time value"]
pub type ScgtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - IrDA/smartcard division value"]
    #[inline(always)]
    pub fn isdiv(&self) -> IsdivR {
        IsdivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Smart card guard time value"]
    #[inline(always)]
    pub fn scgt(&self) -> ScgtR {
        ScgtR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IrDA/smartcard division value"]
    #[inline(always)]
    pub fn isdiv(&mut self) -> IsdivW<'_, GdivSpec> {
        IsdivW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Smart card guard time value"]
    #[inline(always)]
    pub fn scgt(&mut self) -> ScgtW<'_, GdivSpec> {
        ScgtW::new(self, 8)
    }
}
#[doc = "Guard time and division register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GdivSpec;
impl crate::RegisterSpec for GdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdiv::R`](R) reader structure"]
impl crate::Readable for GdivSpec {}
#[doc = "`write(|w| ..)` method takes [`gdiv::W`](W) writer structure"]
impl crate::Writable for GdivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GDIV to value 0"]
impl crate::Resettable for GdivSpec {}
