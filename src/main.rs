mod lexing;

use lexing::token;

fn main() {
    let code = std::fs::read_to_string("examples/hello.wn")
        .expect("Failed to read source file");

    let tokens = lexing::lexer::tokenize(&code);

    for token in tokens {
        println!("{}", token);
    }
}