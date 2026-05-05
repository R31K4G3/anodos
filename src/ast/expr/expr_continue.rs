pub struct ExprContinue {}

impl core::fmt::Debug for ExprContinue {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "continue")
    }
}
