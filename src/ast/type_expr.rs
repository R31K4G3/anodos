use super::expr::ExprIdent;

pub enum TypeInfo<'input> {
    Named(ExprIdent<'input>),
}

impl core::fmt::Debug for TypeInfo<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Named(v) => write!(f, "{:?}", v),
        }
    }
}
