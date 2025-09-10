/// Represents the different types of tokens in the language.
/// These are used by the lexer to categorize pieces of input.
#[derive(Debug, PartialEq)]
enum TokenType {
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

/// Represents a single token
/// 
/// Each token has a `value` (the raw String from the input)
/// and a `token_type` that categorizes its role in the language.
#[derive(Debug, Clone, PartialEq)]
struct Token {
    value: String,
    token_type: TokenType,
}

/// The Lexer struct is responsible for scanning the input string
/// and producing a stream of tokens for the parser to consume.
struct Lexer {
    input: String,                  // The input source code as String
    position: usize,                // The current position in the input
    current_char: Option<char>,     // The character that is currently being examined
}