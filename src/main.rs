mod lexing;

use lexing::token;

use crate::lexing::token::TokenType;

fn main() {
    let code = std::fs::read_to_string("examples/hello.wn")
        .expect("Failed to read source file");

    let mut lexer = lexing::lexer::Lexer::new(&code);

    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        tokens.push(token.clone());

        if matches!(token.token_type, TokenType::EOF) {
            break;
        }
    }

    if !lexer.is_at_end() {
        eprintln!("Lexer did not reach the end of input");
        std::process::exit(1);
    }

    for token in tokens {
        println!("{}", token);
    }
}