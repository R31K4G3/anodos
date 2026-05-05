mod expr_array;
mod expr_binary;
mod expr_block;
mod expr_break;
mod expr_call;
mod expr_construct;
mod expr_continue;
mod expr_ident;
mod expr_if;
mod expr_member_access;
mod expr_return;
mod expr_unary;

use super::lit::Lit;
use super::type_expr::TypeInfo;

pub use expr_array::ExprArray;
pub use expr_binary::{BinaryOp, ExprBinary};
pub use expr_break::ExprBreak;
pub use expr_call::ExprCall;
pub use expr_construct::{ExprConstruct, StructFieldInitializer};
pub use expr_continue::ExprContinue;
pub use expr_ident::ExprIdent;
pub use expr_member_access::ExprMemberAccess;
pub use expr_return::ExprReturn;
pub use expr_unary::{ExprUnary, UnaryOp};

pub enum Expr<'input> {
    ExprArray(ExprArray<'input>),
    ExprBinary(ExprBinary<'input>),
    ExprBreak(ExprBreak),
    ExprCall(ExprCall<'input>),
    ExprConstruct(ExprConstruct<'input>),
    ExprContinue(ExprContinue),
    ExprIdent(ExprIdent<'input>),
    ExprMemberAccess(ExprMemberAccess<'input>),
    ExprReturn(ExprReturn<'input>),
    ExprUnary(ExprUnary<'input>),
    Lit(Lit<'input>),
}

impl<'input> Expr<'input> {
    pub fn array(elements: Vec<Expr<'input>>) -> Self {
        Self::ExprArray(ExprArray { elements })
    }
    pub fn binary(left: Expr<'input>, op: BinaryOp, right: Expr<'input>) -> Self {
        Self::ExprBinary(ExprBinary {
            left: Box::new(left),
            right: Box::new(right),
            op,
        })
    }
    pub fn call(fn_expr: Expr<'input>, args: Vec<Expr<'input>>) -> Self {
        Self::ExprCall(ExprCall {
            fn_expr: Box::new(fn_expr),
            args,
        })
    }
    pub fn construct(
        typename: TypeInfo<'input>,
        fields: Vec<StructFieldInitializer<'input>>,
    ) -> Self {
        Self::ExprConstruct(ExprConstruct { typename, fields })
    }
    pub fn ident(text: &'input str, span: (usize, usize)) -> Self {
        Self::ExprIdent(ExprIdent::new(text, span))
    }
    pub fn member_access(target: Expr<'input>, prop: ExprIdent<'input>) -> Self {
        Self::ExprMemberAccess(ExprMemberAccess {
            target: Box::new(target),
            prop,
        })
    }
    pub fn r#return(expr: Expr<'input>) -> Self {
        Self::ExprReturn(ExprReturn {
            expr: Box::new(expr),
        })
    }
    pub fn unary(op: UnaryOp, expr: Expr<'input>) -> Self {
        Self::ExprUnary(ExprUnary {
            op,
            expr: Box::new(expr),
        })
    }
}

impl Expr<'static> {
    pub fn r#break() -> Self {
        Self::ExprBreak(ExprBreak {})
    }
    pub fn r#continue() -> Self {
        Self::ExprContinue(ExprContinue {})
    }
}

impl core::fmt::Debug for Expr<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ExprArray(v) => write!(f, "{:?}", v),
            Self::ExprBinary(v) => write!(f, "{:?}", v),
            Self::ExprBreak(v) => write!(f, "{:?}", v),
            Self::ExprCall(v) => write!(f, "{:?}", v),
            Self::ExprConstruct(v) => write!(f, "{:?}", v),
            Self::ExprContinue(v) => write!(f, "{:?}", v),
            Self::ExprIdent(v) => write!(f, "{:?}", v),
            Self::ExprMemberAccess(v) => write!(f, "{:?}", v),
            Self::ExprReturn(v) => write!(f, "{:?}", v),
            Self::ExprUnary(v) => write!(f, "{:?}", v),
            Self::Lit(v) => write!(f, "{:?}", v),
        }
    }
}
