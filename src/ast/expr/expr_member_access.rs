use super::{Expr, ExprIdent};

pub struct ExprMemberAccess<'input> {
    pub target: Box<Expr<'input>>,
    pub prop: ExprIdent<'input>,
}

impl core::fmt::Debug for ExprMemberAccess<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({:?}.{:?})", self.target, self.prop)
    }
}
