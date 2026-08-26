#[doc = "Register `IDT` reader"]
pub type R = crate::R<IdtSpec>;
#[doc = "Register `IDT` writer"]
pub type W = crate::W<IdtSpec>;
#[doc = "Field `IDT` reader - Initial Data"]
pub type IdtR = crate::FieldReader<u32>;
#[doc = "Field `IDT` writer - Initial Data"]
pub type IdtW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Initial Data"]
    #[inline(always)]
    pub fn idt(&self) -> IdtR {
        IdtR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Initial Data"]
    #[inline(always)]
    pub fn idt(&mut self) -> IdtW<'_, IdtSpec> {
        IdtW::new(self, 0)
    }
}
#[doc = "Initial data register\n\nYou can [`read`](crate::Reg::read) this register and get [`idt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdtSpec;
impl crate::RegisterSpec for IdtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idt::R`](R) reader structure"]
impl crate::Readable for IdtSpec {}
#[doc = "`write(|w| ..)` method takes [`idt::W`](W) writer structure"]
impl crate::Writable for IdtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDT to value 0xffff_ffff"]
impl crate::Resettable for IdtSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
