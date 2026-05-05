use crate::ast::{expr::ExprIdent, stmt::Stmt, type_expr::TypeInfo};

pub struct StmtFunc<'input> {
    pub name: ExprIdent<'input>,
    pub params: Vec<(ExprIdent<'input>, TypeInfo<'input>)>,
    pub return_type: Option<TypeInfo<'input>>,
    pub body: Vec<Stmt<'input>>,
}

impl core::fmt::Debug for StmtFunc<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "function {:?}", self.name)?;
        if self.params.len() >= 1 {
            f.write_str("(")?;
            write!(f, "{:?}: {:?}", self.params[0].0, self.params[0].1)?;
            for i in &self.params[1..] {
                write!(f, ", {:?}: {:?}", i.0, i.1)?;
            }
            f.write_str(") ")?;
        } else {
            f.write_str("() ")?;
        }
        if let Some(return_type) = &self.return_type {
            f.write_str("-> ")?;
            write!(f, "{:?}", return_type)?;
        }
        f.write_str("{")?;
        for i in &self.body {
            write!(f, "{:?}", i)?;
        }
        f.write_str("}")?;
        Ok(())
    }
}
