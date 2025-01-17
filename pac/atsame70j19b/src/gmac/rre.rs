#[doc = "Register `RRE` reader"]
pub struct R(crate::R<RRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXRER` reader - Receive Resource Errors"]
pub type RXRER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:17 - Receive Resource Errors"]
    #[inline(always)]
    pub fn rxrer(&self) -> RXRER_R {
        RXRER_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
#[doc = "Receive Resource Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rre](index.html) module"]
pub struct RRE_SPEC;
impl crate::RegisterSpec for RRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rre::R](R) reader structure"]
impl crate::Readable for RRE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RRE to value 0"]
impl crate::Resettable for RRE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
