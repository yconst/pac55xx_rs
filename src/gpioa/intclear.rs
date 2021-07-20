#[doc = "Register `INTCLEAR` reader"]
pub struct R(crate::R<INTCLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTCLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTCLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTCLEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTCLEAR` writer"]
pub struct W(crate::W<INTCLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTCLEAR_SPEC>;
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
impl From<crate::W<INTCLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTCLEAR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO A Interrupt Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclear](index.html) module"]
pub struct INTCLEAR_SPEC;
impl crate::RegisterSpec for INTCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intclear::R](R) reader structure"]
impl crate::Readable for INTCLEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intclear::W](W) writer structure"]
impl crate::Writable for INTCLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTCLEAR to value 0"]
impl crate::Resettable for INTCLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}