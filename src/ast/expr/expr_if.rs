use super::Expr;

#[allow(dead_code)]
pub struct ExprIf<'input> {
    pub cond: Box<Expr<'input>>,
    pub then_branch: Box<Expr<'input>>,
    pub else_branch: Box<Expr<'input>>,
}

impl core::fmt::Debug for ExprIf<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "(if {:?}: {:?} else {:?})",
            self.cond, self.then_branch, self.else_branch
        )
    }
}
