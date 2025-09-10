use crate::token::{Token, TokenType};

/// The Lexer struct is responsible for scanning the input string
/// and producing a stream of tokens for the parser to consume.
struct Lexer {
    input: String,                  // The input source code as String
    position: usize,                // The current position in the input
    current_char: Option<char>,     // The character that is currently being examined
}

/// Tokenizes the given source code String into a vector of tokens.
/// 
/// This function scans the input and returns a list of `Token`
/// instances, each representing a meaningful unit in the language.
pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    tokens
}