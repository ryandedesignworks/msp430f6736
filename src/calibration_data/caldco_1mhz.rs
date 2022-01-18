#[doc = "Register `CALDCO_1MHZ` reader"]
pub struct R(crate::R<CALDCO_1MHZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALDCO_1MHZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALDCO_1MHZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALDCO_1MHZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALDCO_1MHZ` writer"]
pub struct W(crate::W<CALDCO_1MHZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALDCO_1MHZ_SPEC>;
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
impl From<crate::W<CALDCO_1MHZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALDCO_1MHZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALDCO_1MHZ` reader - DCOCTL Calibration Data for 1MHz register"]
pub struct CALDCO_1MHZ_R(crate::FieldReader<u8, u8>);
impl CALDCO_1MHZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CALDCO_1MHZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALDCO_1MHZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALDCO_1MHZ` writer - DCOCTL Calibration Data for 1MHz register"]
pub struct CALDCO_1MHZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CALDCO_1MHZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data for 1MHz register"]
    #[inline(always)]
    pub fn caldco_1mhz(&self) -> CALDCO_1MHZ_R {
        CALDCO_1MHZ_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DCOCTL Calibration Data for 1MHz register"]
    #[inline(always)]
    pub fn caldco_1mhz(&mut self) -> CALDCO_1MHZ_W {
        CALDCO_1MHZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "DCOCTL Calibration Data for 1MHz\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caldco_1mhz](index.html) module"]
pub struct CALDCO_1MHZ_SPEC;
impl crate::RegisterSpec for CALDCO_1MHZ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [caldco_1mhz::R](R) reader structure"]
impl crate::Readable for CALDCO_1MHZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [caldco_1mhz::W](W) writer structure"]
impl crate::Writable for CALDCO_1MHZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALDCO_1MHZ to value 0"]
impl crate::Resettable for CALDCO_1MHZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
