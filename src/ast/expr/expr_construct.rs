use crate::ast::type_expr::TypeInfo;

use super::{Expr, ExprIdent};

pub struct StructFieldInitializer<'input> {
    pub name: ExprIdent<'input>,
    pub value: Expr<'input>,
}

impl<'input> StructFieldInitializer<'input> {
    pub fn new(name: ExprIdent<'input>, value: Expr<'input>) -> Self {
        Self { name, value }
    }
}

pub struct ExprConstruct<'input> {
    pub typename: TypeInfo<'input>,
    pub fields: Vec<StructFieldInitializer<'input>>,
}

impl core::fmt::Debug for ExprConstruct<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "construct {:?} ", self.typename)?;
        if self.fields.len() >= 1 {
            f.write_str("{")?;
            write!(f, "{:?}: {:?}", self.fields[0].name, self.fields[0].value)?;
            for i in &self.fields[1..] {
                write!(f, ", {:?}: {:?}", i.name, i.value)?;
            }
            f.write_str("}")?;
        } else {
            f.write_str("{}")?;
        }
        Ok(())
    }
}
