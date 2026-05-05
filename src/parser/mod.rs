mod expr;
mod macros;

use crate::ast::expr::ExprIdent;
use crate::ast::stmt::{
    Stmt, StmtBlock, StmtExpr, StmtFunc, StmtIf, StmtLet, StmtStruct, StmtWhile,
};
use crate::ast::type_expr::TypeInfo;
use crate::lexer::TokenWalker;
use crate::token::{Token, TokenKind};
use macros::{consume_token, expect_token};

#[derive(Debug)]
pub struct ParserError<'input> {
    pub kind: ParserErrorKind<'input>,
}

impl ParserError<'static> {
    #[must_use]
    #[inline(always)]
    pub(super) fn unexpected_eof() -> Self {
        Self {
            kind: ParserErrorKind::UnexpectedEOF,
        }
    }
}

impl<'input> ParserError<'input> {
    #[must_use]
    #[inline(always)]
    pub(super) fn from_unexpected_token(token: Token<'input>) -> Self {
        Self {
            kind: ParserErrorKind::UnexpectedToken(token),
        }
    }
}

#[derive(Debug)]
pub enum ParserErrorKind<'input> {
    UnexpectedToken(Token<'input>),
    UnexpectedEOF,
}

#[must_use]
fn parse_statement<'input>(
    walker: &mut TokenWalker<'input>,
    errors: &mut Vec<()>,
) -> Result<Stmt<'input>, ParserError<'input>> {
    if consume_token!(walker, TokenKind::FunctionKeyword).is_some() {
        let funcname = ExprIdent::from_token(expect_token!(walker, TokenKind::Identifier));
        expect_token!(walker, TokenKind::OpenParen);
        let mut params: Vec<(ExprIdent<'input>, TypeInfo<'input>)> = Vec::new();
        if consume_token!(walker, TokenKind::CloseParen).is_none() {
            let param_name = ExprIdent::from_token(expect_token!(walker, TokenKind::Identifier));
            expect_token!(walker, TokenKind::Colon);
            let param_type = ExprIdent::from_token(expect_token!(walker, TokenKind::Identifier));
            params.push((param_name, TypeInfo::Named(param_type)));
            while consume_token!(walker, TokenKind::Comma).is_some() {
                let param_name =
                    ExprIdent::from_token(expect_token!(walker, TokenKind::Identifier));
                expect_token!(walker, TokenKind::Colon);
                let param_type =
                    ExprIdent::from_token(expect_token!(walker, TokenKind::Identifier));
                params.push((param_name, TypeInfo::Named(param_type)));
            }
            expect_token!(walker, TokenKind::CloseParen);
        }
        let return_type = if consume_token!(walker, TokenKind::ThinArrow).is_some() {
            let return_type = ExprIdent::from_token(expect_token!(walker, TokenKind::Identifier));
            Some(TypeInfo::Named(return_type))
        } else {
            None
        };
        expect_token!(walker, TokenKind::OpenCurly);
        let mut body = Vec::new();
        while consume_token!(walker, TokenKind::CloseCurly).is_none() {
            body.push(parse_statement(walker, errors)?);
        }
        return Ok(Stmt::StmtFunc(StmtFunc {
            name: funcname,
            params,
            return_type,
            body,
        }));
    }

    if consume_token!(walker, TokenKind::StructKeyword).is_some() {
        let typename = ExprIdent::from_token(expect_token!(walker, TokenKind::Identifier));
        expect_token!(walker, TokenKind::OpenCurly);
        let mut fields: Vec<(ExprIdent<'input>, TypeInfo<'input>)> = Vec::new();
        if consume_token!(walker, TokenKind::CloseParen).is_none() {
            let field_name = ExprIdent::from_token(expect_token!(walker, TokenKind::Identifier));
            expect_token!(walker, TokenKind::Colon);
            let field_type = ExprIdent::from_token(expect_token!(walker, TokenKind::Identifier));
            fields.push((field_name, TypeInfo::Named(field_type)));
            while consume_token!(walker, TokenKind::Comma).is_some() {
                let field_name =
                    ExprIdent::from_token(expect_token!(walker, TokenKind::Identifier));
                expect_token!(walker, TokenKind::Colon);
                let field_type =
                    ExprIdent::from_token(expect_token!(walker, TokenKind::Identifier));
                fields.push((field_name, TypeInfo::Named(field_type)));
            }
            expect_token!(walker, TokenKind::CloseParen);
        }
        return Ok(Stmt::StmtStruct(StmtStruct {
            name: typename,
            fields,
        }));
    }

    if consume_token!(walker, TokenKind::WhileKeyword).is_some() {
        let cond = Box::new(expr::parse_expression(walker, errors)?);
        expect_token!(walker, TokenKind::Colon);
        let body = Box::new(parse_statement(walker, errors)?);
        return Ok(Stmt::StmtWhile(StmtWhile { cond, branch: body }));
    }

    if consume_token!(walker, TokenKind::LetKeyword).is_some() {
        let is_mutable = consume_token!(walker, TokenKind::MutableKeyword).is_some();
        let var_name = expect_token!(walker, TokenKind::Identifier);
        expect_token!(walker, TokenKind::SingleEqual);
        let var_init = expr::parse_expression(walker, errors)?;
        expect_token!(walker, TokenKind::SemiColon);
        return Ok(Stmt::StmtLet(StmtLet {
            is_mutable,
            name: ExprIdent::from_token(var_name),
            init: var_init,
        }));
    }

    if consume_token!(walker, TokenKind::IfKeyword).is_some() {
        let cond = Box::new(expr::parse_expression(walker, errors)?);
        expect_token!(walker, TokenKind::Colon);
        let then_branch = Box::new(parse_statement(walker, errors)?);
        let else_branch = if consume_token!(walker, TokenKind::ElseKeyword).is_some() {
            Some(Box::new(parse_statement(walker, errors)?))
        } else {
            None
        };
        return Ok(Stmt::StmtIf(StmtIf {
            cond,
            then_branch,
            else_branch,
        }));
    }

    if consume_token!(walker, TokenKind::OpenCurly).is_some() {
        let mut items = Vec::new();
        while consume_token!(walker, TokenKind::CloseCurly).is_none() {
            items.push(parse_statement(walker, errors)?);
        }
        return Ok(Stmt::StmtBlock(StmtBlock { items }));
    }

    let expr = expr::parse_expression(walker, errors)?;
    expect_token!(walker, TokenKind::SemiColon);
    Ok(Stmt::StmtExpr(StmtExpr {
        expr: Box::new(expr),
    }))
}

#[inline(never)]
#[must_use]
pub fn parse<'input>(mut walker: TokenWalker<'input>) -> Result<Stmt<'input>, ParserError<'input>> {
    let mut items = Vec::new();
    let mut errors = Vec::new();
    while walker.peek().is_some() {
        items.push(parse_statement(&mut walker, &mut errors)?);
    }
    Ok(Stmt::StmtBlock(StmtBlock { items }))
}
