use crate::ast::stmt::Stmt;

use super::Expr;

#[allow(dead_code)]
pub struct ExprBlock<'input> {
    pub items: Vec<Stmt<'input>>,
    pub last: Box<Expr<'input>>,
}

impl core::fmt::Debug for ExprBlock<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("{")?;
        for i in &self.items {
            write!(f, "{:?}", i)?;
        }
        write!(f, "{:?}", self.last)?;
        f.write_str("}")?;
        Ok(())
    }
}
