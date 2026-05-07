use super::Expr;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnaryOp {
    Negate,
    Not,
    BitNot,
}

pub struct ExprUnary<'input> {
    pub op: UnaryOp,
    pub expr: Box<Expr<'input>>,
}

impl core::fmt::Debug for ExprUnary<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({:?}{:?})", self.op, self.expr)
    }
}
