macro_rules! expect_token {
    ($walker:expr, $pattern:pat) => {{
        let c = match $walker.peek().map(|t| &t.kind) {
            Some(_t) => {
                if matches!(_t, $pattern) {
                    $walker.next()
                } else {
                    None
                }
            }
            None => None,
        };
        match c {
            Some(v) => v,
            None => Err(match $walker.next() {
                Some(t) => ParserError::from_unexpected_token(t),
                None => ParserError::unexpected_eof(),
            })?,
        }
    }};
}

macro_rules! consume_token {
    ($walker:expr, $pattern:pat) => {
        match $walker.peek().map(|t| &t.kind) {
            Some(_t) => {
                if matches!(_t, $pattern) {
                    $walker.next()
                } else {
                    None
                }
            }
            None => None,
        }
    };
}

pub(crate) use {consume_token, expect_token};
