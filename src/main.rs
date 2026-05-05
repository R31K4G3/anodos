mod ast;
mod lexer;
mod parser;
mod token;

use parser::ParserErrorKind;

fn main() {
    let file_content = std::fs::read_to_string("test.anod").unwrap();
    println!(
        "{:?}",
        lexer::TokenWalker::new(&file_content).collect::<Vec<_>>()
    );
    match parser::parse(lexer::TokenWalker::new(&file_content)) {
        Ok(parsed) => println!("{:?}", parsed),
        Err(err) => match err.kind {
            ParserErrorKind::UnexpectedEOF => {
                ariadne::Report::build(
                    ariadne::ReportKind::Error,
                    ("test.anod", (file_content.len() - 1)..file_content.len()),
                )
                .with_message("Unexpected EOF")
                .with_label(
                    ariadne::Label::new((
                        "test.anod",
                        (file_content.len() - 1)..file_content.len(),
                    ))
                    .with_color(ariadne::Color::Red),
                )
                .finish()
                .print((
                    "test.anod",
                    ariadne::Source::from(std::fs::read_to_string("test.anod").unwrap()),
                ))
                .unwrap();
            }
            ParserErrorKind::UnexpectedToken(token) => {
                ariadne::Report::build(
                    ariadne::ReportKind::Error,
                    ("test.anod", token.span.0..token.span.1),
                )
                .with_message(format!("Unexpected {:?} Token", token.kind))
                .with_label(
                    ariadne::Label::new(("test.anod", token.span.0..token.span.1))
                        .with_color(ariadne::Color::Red),
                )
                .finish()
                .print((
                    "test.anod",
                    ariadne::Source::from(std::fs::read_to_string("test.anod").unwrap()),
                ))
                .unwrap();
            }
        },
    }
}
