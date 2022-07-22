#[doc = "Register `UCA0IFG` reader"]
pub struct R(crate::R<UCA0IFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA0IFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA0IFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA0IFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA0IFG` writer"]
pub struct W(crate::W<UCA0IFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA0IFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<UCA0IFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA0IFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCRXIFG` reader - receive interrupt flag"]
pub type UCRXIFG_R = crate::BitReader<bool>;
#[doc = "Field `UCRXIFG` writer - receive interrupt flag"]
pub type UCRXIFG_W<'a> = crate::BitWriter<'a, u8, UCA0IFG_SPEC, bool, 0>;
#[doc = "Field `UCTXIFG` reader - transmit interrupt flag"]
pub type UCTXIFG_R = crate::BitReader<bool>;
#[doc = "Field `UCTXIFG` writer - transmit interrupt flag"]
pub type UCTXIFG_W<'a> = crate::BitWriter<'a, u8, UCA0IFG_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - receive interrupt flag"]
    #[inline(always)]
    pub fn ucrxifg(&self) -> UCRXIFG_R {
        UCRXIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - transmit interrupt flag"]
    #[inline(always)]
    pub fn uctxifg(&self) -> UCTXIFG_R {
        UCTXIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - receive interrupt flag"]
    #[inline(always)]
    pub fn ucrxifg(&mut self) -> UCRXIFG_W {
        UCRXIFG_W::new(self)
    }
    #[doc = "Bit 1 - transmit interrupt flag"]
    #[inline(always)]
    pub fn uctxifg(&mut self) -> UCTXIFG_W {
        UCTXIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca0ifg](index.html) module"]
pub struct UCA0IFG_SPEC;
impl crate::RegisterSpec for UCA0IFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca0ifg::R](R) reader structure"]
impl crate::Readable for UCA0IFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca0ifg::W](W) writer structure"]
impl crate::Writable for UCA0IFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCA0IFG to value 0"]
impl crate::Resettable for UCA0IFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
