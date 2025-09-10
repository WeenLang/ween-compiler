/// Represents the different kinds of tokens in the language.
/// These are used by the lexer to categorize pieces of input.
#[derive(Debug, PartialEq)]
enum Token {
    Keyword(String),        // e.g., "def"
    Identifier(String),     // e.g., "header", "font-style"
    StrinLiteral(String),   // e.g., "Hello, World!"
    Number(f64),            // e.g., 11, 29, 1342
    Equals,                 // e.g., '='
    Comma,                  // e.g., ','
    Semicolon,              // e.g., ';'
    LParen, RParen,         // e.g., '(' and ')'
    LBrace, RBrace,         // e.g., '{' and '}'
    LessThan, GreaterThan,  // e.g., '<' and '>'
    EOF,                    // End of input
}