use crate::ast::expr::{Expr, ExprIdent};

pub struct StmtLet<'input> {
    pub is_mutable: bool,
    pub name: ExprIdent<'input>,
    pub init: Expr<'input>,
}

impl core::fmt::Debug for StmtLet<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "let {} {:?} = {:?};",
            if self.is_mutable { "mutable" } else { "" },
            self.name,
            self.init
        )
    }
}
