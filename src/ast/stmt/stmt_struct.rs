use crate::ast::{expr::ExprIdent, type_expr::TypeInfo};

pub struct StmtStruct<'input> {
    pub name: ExprIdent<'input>,
    pub fields: Vec<(ExprIdent<'input>, TypeInfo<'input>)>,
}

impl core::fmt::Debug for StmtStruct<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "struct {:?} ", self.name)?;
        if self.fields.len() >= 1 {
            f.write_str("{ ")?;
            write!(f, "{:?}: {:?}", self.fields[0].0, self.fields[0].1)?;
            for i in &self.fields[1..] {
                write!(f, ", {:?}: {:?}", i.0, i.1)?;
            }
            f.write_str(" }")?;
        } else {
            f.write_str("{}")?;
        }
        Ok(())
    }
}
