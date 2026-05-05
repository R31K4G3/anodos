mod stmt_block;
mod stmt_expr;
mod stmt_func;
mod stmt_if;
mod stmt_let;
mod stmt_struct;
mod stmt_while;

pub use stmt_block::StmtBlock;
pub use stmt_expr::StmtExpr;
pub use stmt_func::StmtFunc;
pub use stmt_if::StmtIf;
pub use stmt_let::StmtLet;
pub use stmt_struct::StmtStruct;
pub use stmt_while::StmtWhile;

pub enum Stmt<'input> {
    StmtBlock(StmtBlock<'input>),
    StmtExpr(StmtExpr<'input>),
    StmtFunc(StmtFunc<'input>),
    StmtIf(StmtIf<'input>),
    StmtLet(StmtLet<'input>),
    StmtStruct(StmtStruct<'input>),
    StmtWhile(StmtWhile<'input>),
}

impl core::fmt::Debug for Stmt<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::StmtBlock(v) => write!(f, "{:?}", v),
            Self::StmtExpr(v) => write!(f, "{:?}", v),
            Self::StmtFunc(v) => write!(f, "{:?}", v),
            Self::StmtIf(v) => write!(f, "{:?}", v),
            Self::StmtLet(v) => write!(f, "{:?}", v),
            Self::StmtStruct(v) => write!(f, "{:?}", v),
            Self::StmtWhile(v) => write!(f, "{:?}", v),
        }
    }
}
