mod lexer;
mod token;

fn main() {
    for a in lexer::TokenWalker::new(&std::fs::read_to_string("test.anod").unwrap()) {
        println!("{:?}", a);
    }
}
