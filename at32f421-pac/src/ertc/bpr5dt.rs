#[doc = "Register `BPR5DT` reader"]
pub type R = crate::R<Bpr5dtSpec>;
#[doc = "Register `BPR5DT` writer"]
pub type W = crate::W<Bpr5dtSpec>;
#[doc = "Field `DT` reader - Battery powered domain data"]
pub type DtR = crate::FieldReader<u32>;
#[doc = "Field `DT` writer - Battery powered domain data"]
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Battery powered domain data"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Battery powered domain data"]
    #[inline(always)]
    pub fn dt(&mut self) -> DtW<'_, Bpr5dtSpec> {
        DtW::new(self, 0)
    }
}
#[doc = "Battery powered domain register\n\nYou can [`read`](crate::Reg::read) this register and get [`bpr5dt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpr5dt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bpr5dtSpec;
impl crate::RegisterSpec for Bpr5dtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bpr5dt::R`](R) reader structure"]
impl crate::Readable for Bpr5dtSpec {}
#[doc = "`write(|w| ..)` method takes [`bpr5dt::W`](W) writer structure"]
impl crate::Writable for Bpr5dtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BPR5DT to value 0"]
impl crate::Resettable for Bpr5dtSpec {}
