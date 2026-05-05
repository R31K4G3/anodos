use super::Expr;

pub struct ExprCall<'input> {
    pub fn_expr: Box<Expr<'input>>,
    pub args: Vec<Expr<'input>>,
}

impl core::fmt::Debug for ExprCall<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.fn_expr)?;
        if self.args.len() >= 1 {
            f.write_str("(")?;
            write!(f, "{:?}", self.args[0])?;
            for i in &self.args[1..] {
                write!(f, ", {:?}", i)?;
            }
            f.write_str(")")?;
        } else {
            f.write_str("()")?;
        }
        Ok(())
    }
}
