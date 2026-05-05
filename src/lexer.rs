use crate::token::{Token, TokenKind};
use itertools::{EitherOrBoth, Itertools as _};
use std::sync::LazyLock;

static KEYWORDS: LazyLock<[(TokenKind, &'static str); 51]> = LazyLock::new(|| {
    let mut keywords = [
        (TokenKind::AsKeyword, "as"),
        (TokenKind::AsyncKeyword, "async"),
        (TokenKind::AwaitKeyword, "await"),
        (TokenKind::BreakKeyword, "break"),
        (TokenKind::CaseKeyword, "case"),
        (TokenKind::CatchKeyword, "catch"),
        (TokenKind::ClassKeyword, "class"),
        (TokenKind::ConstKeyword, "const"),
        (TokenKind::ConstructKeyword, "construct"),
        (TokenKind::ContinueKeyword, "continue"),
        (TokenKind::DataKeyword, "data"),
        (TokenKind::DefaultKeyword, "default"),
        (TokenKind::DoKeyword, "do"),
        (TokenKind::DynamicKeyword, "dynamic"),
        (TokenKind::ElseKeyword, "else"),
        (TokenKind::EnumKeyword, "enum"),
        (TokenKind::ExportKeyword, "export"),
        (TokenKind::FalseKeyword, "false"),
        (TokenKind::FinallyKeyword, "finally"),
        (TokenKind::FunctionKeyword, "function"),
        (TokenKind::ForKeyword, "for"),
        (TokenKind::ForeachKeyword, "foreach"),
        (TokenKind::GlobalKeyword, "global"),
        (TokenKind::IfKeyword, "if"),
        (TokenKind::ImplKeyword, "impl"),
        (TokenKind::ImportKeyword, "import"),
        (TokenKind::InKeyword, "in"),
        (TokenKind::InstanceKeyword, "instance"),
        (TokenKind::IsKeyword, "is"),
        (TokenKind::LetKeyword, "let"),
        (TokenKind::LocalKeyword, "local"),
        (TokenKind::MatchKeyword, "match"),
        (TokenKind::ModuleKeyword, "module"),
        (TokenKind::MoveKeyword, "move"),
        (TokenKind::MutableKeyword, "mutable"),
        (TokenKind::OfKeyword, "of"),
        (TokenKind::ReturnKeyword, "return"),
        (TokenKind::SelfKeyword, "self"),
        (TokenKind::StaticKeyword, "static"),
        (TokenKind::StructKeyword, "struct"),
        (TokenKind::ThenKeyword, "then"),
        (TokenKind::ThrowKeyword, "throw"),
        (TokenKind::TraitKeyword, "trait"),
        (TokenKind::TrueKeyword, "true"),
        (TokenKind::TryKeyword, "try"),
        (TokenKind::TypeKeyword, "type"),
        (TokenKind::TypeofKeyword, "typeof"),
        (TokenKind::VarKeyword, "var"),
        (TokenKind::WhereKeyword, "where"),
        (TokenKind::WhileKeyword, "while"),
        (TokenKind::YieldKeyword, "yield"),
    ];
    keywords.sort_by(|(_, a), (_, b)| usize::cmp(&b.len(), &a.len()));
    keywords
});

static SYMBOLS: LazyLock<[(TokenKind, &'static str); 45]> = LazyLock::new(|| {
    let mut symbols = [
        (TokenKind::SingleEqual, "="),
        (TokenKind::DoubleEqual, "=="),
        (TokenKind::ThickArrow, "=>"),
        (TokenKind::SemiColon, ";"),
        (TokenKind::Colon, ":"),
        (TokenKind::DoubleColon, "::"),
        (TokenKind::Dot, "."),
        (TokenKind::Comma, ","),
        (TokenKind::Tilde, "~"),
        (TokenKind::ExclamationEq, "!="),
        (TokenKind::PlusEq, "+="),
        (TokenKind::MinusEq, "-="),
        (TokenKind::ThinArrow, "->"),
        (TokenKind::AsteriskEq, "*="),
        (TokenKind::SlashEq, "/="),
        (TokenKind::PercentEq, "%="),
        (TokenKind::LessThan, "<"),
        (TokenKind::DoubleLessThan, "<<"),
        (TokenKind::LessThanEq, "<="),
        (TokenKind::DoubleLessThanEq, "<<="),
        (TokenKind::GreaterThan, ">"),
        (TokenKind::DoubleGreaterThan, ">>"),
        (TokenKind::GreaterThanEq, ">="),
        (TokenKind::DoubleGreaterThanEq, ">>="),
        (TokenKind::Plus, "+"),
        (TokenKind::DoublePlus, "++"),
        (TokenKind::Minus, "-"),
        (TokenKind::Asterisk, "*"),
        (TokenKind::Slash, "/"),
        (TokenKind::Percent, "%"),
        (TokenKind::LeftParen, "("),
        (TokenKind::RightParen, ")"),
        (TokenKind::LeftCurly, "{"),
        (TokenKind::RightCurly, "}"),
        (TokenKind::LeftBracket, "["),
        (TokenKind::RightBracket, "]"),
        (TokenKind::Exclamation, "!"),
        (TokenKind::Ampersand, "&"),
        (TokenKind::AmpersandEq, "&="),
        (TokenKind::Pipe, "|"),
        (TokenKind::PipeEq, "|="),
        (TokenKind::Circumflex, "^"),
        (TokenKind::CircumflexEq, "^="),
        (TokenKind::DoubleAmpersand, "&&"),
        (TokenKind::DoublePipe, "||"),
    ];
    symbols.sort_by(|(_, a), (_, b)| usize::cmp(&b.len(), &a.len()));
    symbols
});

pub struct Lexer<'input> {
    source: &'input str,
    current_index: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(source: &'input str) -> Self {
        Self {
            source,
            current_index: 0,
        }
    }
}

fn is_ident_start(c: char) -> bool {
    c == '_' || c.is_ascii_alphabetic()
}

fn is_ident_part(c: char) -> bool {
    c == '_' || c.is_ascii_alphanumeric()
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token<'input>;
    fn next(&mut self) -> Option<Self::Item> {
        let source = {
            let white_spaces = self.source[self.current_index..]
                .as_bytes()
                .iter()
                .take_while(|c| matches!(*c, b'\t' | b'\n' | b'\r' | b' '))
                .count();
            self.current_index += white_spaces;
            let source = &self.source[self.current_index..];
            if source.is_empty() {
                return None;
            }
            source
        };

        for &(kind, s) in LazyLock::force(&SYMBOLS) {
            let is_same = s.chars().zip(source.chars()).all(|(a, b)| a == b);
            if is_same {
                let token = Token {
                    kind,
                    text: self.source,
                    span: (self.current_index, self.current_index + s.len()),
                };
                self.current_index += s.len();
                return Some(token);
            }
        }
        let first_char = source.chars().next().unwrap();
        if is_ident_start(first_char) {
            'outer: for &(kind, s) in LazyLock::force(&KEYWORDS) {
                let ident = source
                    .chars()
                    .enumerate()
                    .take_while(|&(i, c)| {
                        if i == 0 {
                            is_ident_start(c)
                        } else {
                            is_ident_part(c)
                        }
                    })
                    .map(|(_, c)| c);
                for v in s.chars().zip_longest(ident) {
                    match v {
                        EitherOrBoth::Both(a, b) => {
                            if a != b {
                                continue 'outer;
                            }
                        }
                        EitherOrBoth::Left(_) => break,
                        EitherOrBoth::Right(_) => continue 'outer,
                    }
                }
                let token = Token {
                    kind,
                    text: self.source,
                    span: (self.current_index, self.current_index + s.len()),
                };
                self.current_index += s.len();
                return Some(token);
            }
            let ident_len = source
                .chars()
                .enumerate()
                .take_while(|&(i, c)| {
                    if i == 0 {
                        is_ident_start(c)
                    } else {
                        is_ident_part(c)
                    }
                })
                .map(|(_, c)| c)
                .count();
            let token = Token {
                kind: TokenKind::Identifier,
                text: self.source,
                span: (self.current_index, self.current_index + ident_len),
            };
            self.current_index += ident_len;
            return Some(token);
        }
        if first_char == '"' {
            let mut len = 1;
            for c in source.chars().skip(1) {
                len += c.len_utf8();
                if c == '"' {
                    let token = Token {
                        kind: TokenKind::StrLiteral,
                        text: self.source,
                        span: (self.current_index, self.current_index + len),
                    };
                    self.current_index += len;
                    return Some(token);
                }
            }
            let token = Token {
                kind: TokenKind::UnterminatedLiteral,
                text: self.source,
                span: (self.current_index, self.current_index + len),
            };
            self.current_index += len;
            return Some(token);
        }
        if first_char == '\'' {
            let mut len = 1;
            for c in source.chars().skip(1) {
                len += c.len_utf8();
                if c == '\'' {
                    let token = Token {
                        kind: TokenKind::CharLiteral,
                        text: self.source,
                        span: (self.current_index, self.current_index + len),
                    };
                    self.current_index += len;
                    return Some(token);
                }
            }
            let token = Token {
                kind: TokenKind::UnterminatedLiteral,
                text: self.source,
                span: (self.current_index, self.current_index + len),
            };
            self.current_index += len;
            return Some(token);
        }
        if first_char.is_ascii_digit() {
            let num_len = source.chars().take_while(char::is_ascii_digit).count();
            let num_len = if source[num_len..].chars().next().is_some_and(|c| c == '.') {
                num_len
                    + 1
                    + source[(num_len + 1)..]
                        .chars()
                        .take_while(char::is_ascii_digit)
                        .count()
            } else {
                num_len
            };
            let token = Token {
                kind: TokenKind::NumLiteral,
                text: self.source,
                span: (self.current_index, self.current_index + num_len),
            };
            self.current_index += num_len;
            return Some(token);
        }
        let token = Token {
            kind: TokenKind::UnexpectedCharacter,
            text: self.source,
            span: (
                self.current_index,
                self.current_index + first_char.len_utf8(),
            ),
        };
        self.current_index += first_char.len_utf8();
        return Some(token);
    }
}
