use super::Stmt;
use crate::ast::expr::Expr;

pub struct StmtWhile<'input> {
    pub cond: Box<Expr<'input>>,
    pub branch: Box<Stmt<'input>>,
}

impl core::fmt::Debug for StmtWhile<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "while {:?}: {:?}", self.cond, self.branch)
    }
}
