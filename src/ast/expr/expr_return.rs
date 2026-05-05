use super::Expr;

#[repr(transparent)]
pub struct ExprReturn<'input> {
    pub expr: Box<Expr<'input>>,
}

impl core::fmt::Debug for ExprReturn<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(return {:?})", self.expr)
    }
}
