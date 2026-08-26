#[doc = "Register `WP` writer"]
pub type W = crate::W<WpSpec>;
#[doc = "Field `CMD` writer - Command register"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Command register"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, WpSpec> {
        CmdW::new(self, 0)
    }
}
#[doc = "write protection register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wp::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpSpec;
impl crate::RegisterSpec for WpSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wp::W`](W) writer structure"]
impl crate::Writable for WpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WP to value 0"]
impl crate::Resettable for WpSpec {}
