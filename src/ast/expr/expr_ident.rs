use crate::token::{Token, TokenKind};

pub struct ExprIdent<'input> {
    pub text: &'input str,
    pub span: (usize, usize),
}

impl<'input> ExprIdent<'input> {
    #[inline(always)]
    pub fn new(text: &'input str, span: (usize, usize)) -> Self {
        Self { text, span }
    }
    #[inline(always)]
    pub fn from_token(token: Token<'input>) -> Self {
        assert!(matches!(token.kind, TokenKind::Identifier));
        Self { text: token.text, span: token.span }
    }
}

impl core::fmt::Debug for ExprIdent<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            self.text.get(self.span.0..self.span.1).unwrap_or("???")
        )
    }
}
