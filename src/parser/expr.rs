use super::ParserError;
use super::macros::{consume_token, expect_token};
use crate::ast::expr::{BinaryOp, Expr, ExprIdent, StructFieldInitializer, UnaryOp};
use crate::ast::lit::Lit;
use crate::ast::type_expr::TypeInfo;
use crate::lexer::TokenWalker;
use crate::token::TokenKind;

#[must_use]
fn parse_primary<'input>(
    walker: &mut TokenWalker<'input>,
    errors: &mut Vec<()>,
) -> Result<Expr<'input>, ParserError<'input>> {
    if consume_token!(walker, TokenKind::OpenParen).is_some() {
        let expr = parse_expression(walker, errors)?;
        expect_token!(walker, TokenKind::CloseParen);
        return Ok(expr);
    }

    if let Some(token) = consume_token!(walker, TokenKind::Identifier) {
        return Ok(Expr::ident(token.text, token.span));
    }

    if consume_token!(walker, TokenKind::ConstructKeyword).is_some() {
        expect_token!(walker, TokenKind::OpenCurly);
        let typename = expect_token!(walker, TokenKind::Identifier);
        let mut fields = Vec::new();
        if consume_token!(walker, TokenKind::CloseBracket).is_none() {
            let field_name = expect_token!(walker, TokenKind::Identifier);
            expect_token!(walker, TokenKind::Colon);
            fields.push(StructFieldInitializer::new(
                ExprIdent::from_token(field_name),
                parse_expression(walker, errors)?,
            ));
            while consume_token!(walker, TokenKind::Comma).is_some() {
                let field_name = expect_token!(walker, TokenKind::Identifier);
                expect_token!(walker, TokenKind::Colon);
                fields.push(StructFieldInitializer::new(
                    ExprIdent::from_token(field_name),
                    parse_expression(walker, errors)?,
                ));
            }
            expect_token!(walker, TokenKind::CloseBracket);
        }
        return Ok(Expr::construct(
            TypeInfo::Named(ExprIdent::from_token(typename)),
            fields,
        ));
    }

    if consume_token!(walker, TokenKind::BreakKeyword).is_some() {
        return Ok(Expr::r#break());
    }

    if consume_token!(walker, TokenKind::ContinueKeyword).is_some() {
        return Ok(Expr::r#continue());
    }

    if let Some(token) = consume_token!(walker, TokenKind::TrueKeyword) {
        return Ok(Expr::Lit(Lit::bool(true, token.span)));
    }

    if let Some(token) = consume_token!(walker, TokenKind::FalseKeyword) {
        return Ok(Expr::Lit(Lit::bool(false, token.span)));
    }

    if let Some(token) = consume_token!(walker, TokenKind::StrLiteral) {
        return Ok(Expr::Lit(Lit::string(token.text, token.span)));
    }

    if let Some(token) = consume_token!(walker, TokenKind::CharLiteral) {
        return Ok(Expr::Lit(Lit::char(token.text, token.span)));
    }

    if let Some(token) = consume_token!(walker, TokenKind::NumLiteral) {
        return Ok(Expr::Lit(Lit::number(token.text, token.span)));
    }

    if consume_token!(walker, TokenKind::OpenBracket).is_some() {
        let mut entries = Vec::new();
        if consume_token!(walker, TokenKind::CloseBracket).is_none() {
            entries.push(parse_expression(walker, errors)?);
            while consume_token!(walker, TokenKind::Comma).is_some() {
                entries.push(parse_expression(walker, errors)?);
            }
            expect_token!(walker, TokenKind::CloseBracket);
        }
        return Ok(Expr::array(entries));
    }

    Err(match walker.next() {
        Some(t) => ParserError::from_unexpected_token(t),
        None => ParserError::unexpected_eof(),
    })
}

#[must_use]
fn parse_secondary<'input>(
    walker: &mut TokenWalker<'input>,
    errors: &mut Vec<()>,
) -> Result<Expr<'input>, ParserError<'input>> {
    let mut node = parse_primary(walker, errors)?;

    loop {
        if consume_token!(walker, TokenKind::Dot).is_some() {
            let token = expect_token!(walker, TokenKind::Identifier);
            node = Expr::member_access(node, ExprIdent::from_token(token));
        } else if consume_token!(walker, TokenKind::OpenBracket).is_some() {
            let right = parse_expression(walker, errors)?;
            expect_token!(walker, TokenKind::CloseBracket);
            node = Expr::binary(node, BinaryOp::IdxAccess, right);
        } else if consume_token!(walker, TokenKind::OpenParen).is_some() {
            let mut args = Vec::new();
            if consume_token!(walker, TokenKind::CloseParen).is_none() {
                args.push(parse_expression(walker, errors)?);
                while consume_token!(walker, TokenKind::Comma).is_some() {
                    args.push(parse_expression(walker, errors)?);
                }
                expect_token!(walker, TokenKind::CloseParen);
            }
            node = Expr::call(node, args);
        } else {
            return Ok(node);
        }
    }
}

#[must_use]
fn parse_unary<'input>(
    walker: &mut TokenWalker<'input>,
    errors: &mut Vec<()>,
) -> Result<Expr<'input>, ParserError<'input>> {
    let peeked_token = match walker.peek() {
        Some(v) => v,
        None => return parse_secondary(walker, errors),
    };
    let unary_op = match peeked_token.kind {
        TokenKind::Exclamation => UnaryOp::Not,
        TokenKind::Minus => UnaryOp::Negate,
        TokenKind::Tilde => UnaryOp::BitNot,
        _ => return parse_secondary(walker, errors),
    };
    walker.next().unwrap();

    Ok(Expr::unary(unary_op, parse_unary(walker, errors)?))
}

#[must_use]
fn parse_muldiv<'input>(
    walker: &mut TokenWalker<'input>,
    errors: &mut Vec<()>,
) -> Result<Expr<'input>, ParserError<'input>> {
    let mut node = parse_unary(walker, errors)?;

    loop {
        let peeked_token = match walker.peek() {
            Some(v) => v,
            None => return Ok(node),
        };
        let binary_op = match peeked_token.kind {
            TokenKind::Asterisk => BinaryOp::Mul,
            TokenKind::Slash => BinaryOp::Div,
            TokenKind::Percent => BinaryOp::Rem,
            _ => return Ok(node),
        };
        walker.next().unwrap();
        node = Expr::binary(node, binary_op, parse_unary(walker, errors)?);
    }
}

#[must_use]
fn parse_addsub<'input>(
    walker: &mut TokenWalker<'input>,
    errors: &mut Vec<()>,
) -> Result<Expr<'input>, ParserError<'input>> {
    let mut node = parse_muldiv(walker, errors)?;

    loop {
        let peeked_token = match walker.peek() {
            Some(v) => v,
            None => return Ok(node),
        };
        let binary_op = match peeked_token.kind {
            TokenKind::Plus => BinaryOp::Add,
            TokenKind::Minus => BinaryOp::Sub,
            _ => return Ok(node),
        };
        walker.next().unwrap();
        node = Expr::binary(node, binary_op, parse_muldiv(walker, errors)?);
    }
}

#[must_use]
fn parse_bitshift<'input>(
    walker: &mut TokenWalker<'input>,
    errors: &mut Vec<()>,
) -> Result<Expr<'input>, ParserError<'input>> {
    let mut node = parse_addsub(walker, errors)?;

    loop {
        let peeked_token = match walker.peek() {
            Some(v) => v,
            None => return Ok(node),
        };
        let binary_op = match peeked_token.kind {
            TokenKind::DoubleLessThan => BinaryOp::Shl,
            TokenKind::DoubleGreaterThan => BinaryOp::Shr,
            _ => return Ok(node),
        };
        walker.next().unwrap();
        node = Expr::binary(node, binary_op, parse_addsub(walker, errors)?);
    }
}

#[must_use]
fn parse_bitand<'input>(
    walker: &mut TokenWalker<'input>,
    errors: &mut Vec<()>,
) -> Result<Expr<'input>, ParserError<'input>> {
    let mut node = parse_bitshift(walker, errors)?;

    while consume_token!(walker, TokenKind::Ampersand).is_some() {
        node = Expr::binary(node, BinaryOp::BitAnd, parse_bitshift(walker, errors)?);
    }

    Ok(node)
}

#[must_use]
fn parse_bitxor<'input>(
    walker: &mut TokenWalker<'input>,
    errors: &mut Vec<()>,
) -> Result<Expr<'input>, ParserError<'input>> {
    let mut node = parse_bitand(walker, errors)?;

    while consume_token!(walker, TokenKind::Circumflex).is_some() {
        node = Expr::binary(node, BinaryOp::BitXor, parse_bitand(walker, errors)?);
    }

    Ok(node)
}

#[must_use]
fn parse_bitor<'input>(
    walker: &mut TokenWalker<'input>,
    errors: &mut Vec<()>,
) -> Result<Expr<'input>, ParserError<'input>> {
    let mut node = parse_bitxor(walker, errors)?;

    while consume_token!(walker, TokenKind::Pipe).is_some() {
        node = Expr::binary(node, BinaryOp::BitOr, parse_bitxor(walker, errors)?);
    }

    Ok(node)
}

#[must_use]
fn parse_relational<'input>(
    walker: &mut TokenWalker<'input>,
    errors: &mut Vec<()>,
) -> Result<Expr<'input>, ParserError<'input>> {
    let mut node = parse_bitor(walker, errors)?;

    loop {
        let peeked_token = match walker.peek() {
            Some(v) => v,
            None => return Ok(node),
        };
        let binary_op = match peeked_token.kind {
            TokenKind::LessThan => BinaryOp::Lt,
            TokenKind::LessThanEq => BinaryOp::Le,
            TokenKind::GreaterThan => BinaryOp::Gt,
            TokenKind::GreaterThanEq => BinaryOp::Ge,
            _ => return Ok(node),
        };
        walker.next().unwrap();
        node = Expr::binary(node, binary_op, parse_bitor(walker, errors)?);
    }
}

#[must_use]
fn parse_equality<'input>(
    walker: &mut TokenWalker<'input>,
    errors: &mut Vec<()>,
) -> Result<Expr<'input>, ParserError<'input>> {
    let mut node = parse_relational(walker, errors)?;

    loop {
        let peeked_token = match walker.peek() {
            Some(v) => v,
            None => return Ok(node),
        };
        let binary_op = match peeked_token.kind {
            TokenKind::DoubleEqual => BinaryOp::Eq,
            TokenKind::ExclamationEq => BinaryOp::Ne,
            _ => return Ok(node),
        };
        walker.next().unwrap();
        node = Expr::binary(node, binary_op, parse_relational(walker, errors)?);
    }
}

#[must_use]
fn parse_logical_and<'input>(
    walker: &mut TokenWalker<'input>,
    errors: &mut Vec<()>,
) -> Result<Expr<'input>, ParserError<'input>> {
    let mut node = parse_equality(walker, errors)?;

    while consume_token!(walker, TokenKind::DoubleAmpersand).is_some() {
        node = Expr::binary(node, BinaryOp::And, parse_equality(walker, errors)?);
    }

    Ok(node)
}

#[must_use]
fn parse_logical_or<'input>(
    walker: &mut TokenWalker<'input>,
    errors: &mut Vec<()>,
) -> Result<Expr<'input>, ParserError<'input>> {
    let mut node = parse_logical_and(walker, errors)?;

    while consume_token!(walker, TokenKind::DoublePipe).is_some() {
        node = Expr::binary(node, BinaryOp::Or, parse_logical_and(walker, errors)?);
    }

    Ok(node)
}

#[must_use]
fn parse_assignment<'input>(
    walker: &mut TokenWalker<'input>,
    errors: &mut Vec<()>,
) -> Result<Expr<'input>, ParserError<'input>> {
    let node = parse_logical_or(walker, errors)?;
    let peeked_token = match walker.peek() {
        Some(v) => v,
        None => return Ok(node),
    };
    let binary_op = match peeked_token.kind {
        TokenKind::SingleEqual => BinaryOp::Assign,
        TokenKind::PlusEq => BinaryOp::AddAssign,
        TokenKind::MinusEq => BinaryOp::SubAssign,
        TokenKind::AsteriskEq => BinaryOp::MulAssign,
        TokenKind::SlashEq => BinaryOp::DivAssign,
        TokenKind::PercentEq => BinaryOp::RemAssign,
        TokenKind::AmpersandEq => BinaryOp::BitAndAssign,
        TokenKind::PipeEq => BinaryOp::BitOrAssign,
        TokenKind::CircumflexEq => BinaryOp::BitXorAssign,
        TokenKind::DoubleLessThanEq => BinaryOp::ShlAssign,
        TokenKind::DoubleGreaterThanEq => BinaryOp::ShrAssign,
        _ => return Ok(node),
    };
    walker.next().unwrap();
    return Ok(Expr::binary(
        node,
        binary_op,
        parse_assignment(walker, errors)?,
    ));
}

#[must_use]
fn parse_return<'input>(
    walker: &mut TokenWalker<'input>,
    errors: &mut Vec<()>,
) -> Result<Expr<'input>, ParserError<'input>> {
    if consume_token!(walker, TokenKind::ReturnKeyword).is_some() {
        return Ok(Expr::r#return(parse_return(walker, errors)?));
    }
    parse_assignment(walker, errors)
}

#[must_use]
pub(super) fn parse_expression<'input>(
    tokens: &mut TokenWalker<'input>,
    errors: &mut Vec<()>,
) -> Result<Expr<'input>, ParserError<'input>> {
    parse_return(tokens, errors)
}
