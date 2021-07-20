#[doc = "Register `PGMUXSEL` reader"]
pub struct R(crate::R<PGMUXSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PGMUXSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PGMUXSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PGMUXSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PGMUXSEL` writer"]
pub struct W(crate::W<PGMUXSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PGMUXSEL_SPEC>;
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
impl From<crate::W<PGMUXSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PGMUXSEL_SPEC>) -> Self {
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
#[doc = "PG Peripheral MUX Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgmuxsel](index.html) module"]
pub struct PGMUXSEL_SPEC;
impl crate::RegisterSpec for PGMUXSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pgmuxsel::R](R) reader structure"]
impl crate::Readable for PGMUXSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pgmuxsel::W](W) writer structure"]
impl crate::Writable for PGMUXSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PGMUXSEL to value 0"]
impl crate::Resettable for PGMUXSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}