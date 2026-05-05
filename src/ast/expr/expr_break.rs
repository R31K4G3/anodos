pub struct ExprBreak {}

impl core::fmt::Debug for ExprBreak {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "break")
    }
}
