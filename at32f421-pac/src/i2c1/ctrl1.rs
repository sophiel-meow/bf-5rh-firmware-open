#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `I2CEN` reader - Peripheral enable"]
pub type I2cenR = crate::BitReader;
#[doc = "Field `I2CEN` writer - Peripheral enable"]
pub type I2cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERMODE` reader - I2C peripheral mode"]
pub type PermodeR = crate::BitReader;
#[doc = "Field `PERMODE` writer - I2C peripheral mode"]
pub type PermodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBMODE` reader - SMBus device mode"]
pub type SmbmodeR = crate::BitReader;
#[doc = "Field `SMBMODE` writer - SMBus device mode"]
pub type SmbmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARPEN` reader - SMBus address resolution protocol enable"]
pub type ArpenR = crate::BitReader;
#[doc = "Field `ARPEN` writer - SMBus address resolution protocol enable"]
pub type ArpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECEN` reader - PEC calculation enable"]
pub type PecenR = crate::BitReader;
#[doc = "Field `PECEN` writer - PEC calculation enable"]
pub type PecenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCAEN` reader - General call address enable"]
pub type GcaenR = crate::BitReader;
#[doc = "Field `GCAEN` writer - General call address enable"]
pub type GcaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRETCH` reader - Clock stretching mode"]
pub type StretchR = crate::BitReader;
#[doc = "Field `STRETCH` writer - Clock stretching mode"]
pub type StretchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENSTART` reader - Start generation"]
pub type GenstartR = crate::BitReader;
#[doc = "Field `GENSTART` writer - Start generation"]
pub type GenstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENSTOP` reader - Stop generation"]
pub type GenstopR = crate::BitReader;
#[doc = "Field `GENSTOP` writer - Stop generation"]
pub type GenstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKEN` reader - Acknowledge enable"]
pub type AckenR = crate::BitReader;
#[doc = "Field `ACKEN` writer - Acknowledge enable"]
pub type AckenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACKCTRL` reader - Master receiving mode acknowledge control"]
pub type MackctrlR = crate::BitReader;
#[doc = "Field `MACKCTRL` writer - Master receiving mode acknowledge control"]
pub type MackctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECTEN` reader - Request PEC transmission enable"]
pub type PectenR = crate::BitReader;
#[doc = "Field `PECTEN` writer - Request PEC transmission enable"]
pub type PectenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBALERT` reader - SMBus alert pin set"]
pub type SmbalertR = crate::BitReader;
#[doc = "Field `SMBALERT` writer - SMBus alert pin set"]
pub type SmbalertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - I2C peripheral reset"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - I2C peripheral reset"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2cenR {
        I2cenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C peripheral mode"]
    #[inline(always)]
    pub fn permode(&self) -> PermodeR {
        PermodeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBus device mode"]
    #[inline(always)]
    pub fn smbmode(&self) -> SmbmodeR {
        SmbmodeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SMBus address resolution protocol enable"]
    #[inline(always)]
    pub fn arpen(&self) -> ArpenR {
        ArpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PEC calculation enable"]
    #[inline(always)]
    pub fn pecen(&self) -> PecenR {
        PecenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General call address enable"]
    #[inline(always)]
    pub fn gcaen(&self) -> GcaenR {
        GcaenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock stretching mode"]
    #[inline(always)]
    pub fn stretch(&self) -> StretchR {
        StretchR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    pub fn genstart(&self) -> GenstartR {
        GenstartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline(always)]
    pub fn genstop(&self) -> GenstopR {
        GenstopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    pub fn acken(&self) -> AckenR {
        AckenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Master receiving mode acknowledge control"]
    #[inline(always)]
    pub fn mackctrl(&self) -> MackctrlR {
        MackctrlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Request PEC transmission enable"]
    #[inline(always)]
    pub fn pecten(&self) -> PectenR {
        PectenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBus alert pin set"]
    #[inline(always)]
    pub fn smbalert(&self) -> SmbalertR {
        SmbalertR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C peripheral reset"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn i2cen(&mut self) -> I2cenW<'_, Ctrl1Spec> {
        I2cenW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C peripheral mode"]
    #[inline(always)]
    pub fn permode(&mut self) -> PermodeW<'_, Ctrl1Spec> {
        PermodeW::new(self, 1)
    }
    #[doc = "Bit 3 - SMBus device mode"]
    #[inline(always)]
    pub fn smbmode(&mut self) -> SmbmodeW<'_, Ctrl1Spec> {
        SmbmodeW::new(self, 3)
    }
    #[doc = "Bit 4 - SMBus address resolution protocol enable"]
    #[inline(always)]
    pub fn arpen(&mut self) -> ArpenW<'_, Ctrl1Spec> {
        ArpenW::new(self, 4)
    }
    #[doc = "Bit 5 - PEC calculation enable"]
    #[inline(always)]
    pub fn pecen(&mut self) -> PecenW<'_, Ctrl1Spec> {
        PecenW::new(self, 5)
    }
    #[doc = "Bit 6 - General call address enable"]
    #[inline(always)]
    pub fn gcaen(&mut self) -> GcaenW<'_, Ctrl1Spec> {
        GcaenW::new(self, 6)
    }
    #[doc = "Bit 7 - Clock stretching mode"]
    #[inline(always)]
    pub fn stretch(&mut self) -> StretchW<'_, Ctrl1Spec> {
        StretchW::new(self, 7)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    pub fn genstart(&mut self) -> GenstartW<'_, Ctrl1Spec> {
        GenstartW::new(self, 8)
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline(always)]
    pub fn genstop(&mut self) -> GenstopW<'_, Ctrl1Spec> {
        GenstopW::new(self, 9)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    pub fn acken(&mut self) -> AckenW<'_, Ctrl1Spec> {
        AckenW::new(self, 10)
    }
    #[doc = "Bit 11 - Master receiving mode acknowledge control"]
    #[inline(always)]
    pub fn mackctrl(&mut self) -> MackctrlW<'_, Ctrl1Spec> {
        MackctrlW::new(self, 11)
    }
    #[doc = "Bit 12 - Request PEC transmission enable"]
    #[inline(always)]
    pub fn pecten(&mut self) -> PectenW<'_, Ctrl1Spec> {
        PectenW::new(self, 12)
    }
    #[doc = "Bit 13 - SMBus alert pin set"]
    #[inline(always)]
    pub fn smbalert(&mut self) -> SmbalertW<'_, Ctrl1Spec> {
        SmbalertW::new(self, 13)
    }
    #[doc = "Bit 15 - I2C peripheral reset"]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, Ctrl1Spec> {
        ResetW::new(self, 15)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for Ctrl1Spec {}
