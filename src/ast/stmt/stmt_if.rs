use super::Stmt;
use crate::ast::expr::Expr;

pub struct StmtIf<'input> {
    pub cond: Box<Expr<'input>>,
    pub then_branch: Box<Stmt<'input>>,
    pub else_branch: Option<Box<Stmt<'input>>>,
}

impl core::fmt::Debug for StmtIf<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.else_branch {
            Some(else_branch) => {
                write!(
                    f,
                    "if {:?}: {:?} else {:?}",
                    self.cond, self.then_branch, else_branch
                )
            }
            None => {
                write!(f, "if {:?}: {:?}", self.cond, self.then_branch)
            }
        }
    }
}
