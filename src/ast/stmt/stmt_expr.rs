use crate::ast::expr::Expr;

#[repr(transparent)]
pub struct StmtExpr<'input> {
    pub expr: Box<Expr<'input>>,
}

impl core::fmt::Debug for StmtExpr<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?};", self.expr)
    }
}
