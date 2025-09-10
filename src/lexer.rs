/// Represents the different types of tokens in the language.
/// These are used by the lexer to categorize pieces of input.
#[derive(Debug, PartialEq)]
enum TokenType {
    /// A keyword like `def`, `header`, etc.
    Keyword(String),        

    /// An identifier such as variables names or tag names
    Identifier(String),     

    /// A String literal enclosed in double quotes
    StrinLiteral(String),   

    /// A numeric literal, including integers and floats.
    Number(f64),            

    /// Symbols and punctuation
    Equals,                 // '='
    Comma,                  // ','
    Semicolon,              // ';'
    LParen, RParen,         // '(' and ')'
    LBrace, RBrace,         // '{' and '}'
    LessThan, GreaterThan,  // '<' and '>'

    /// End of input marker
    EOF,

    /// Unknown or invalid token
    /// Useful for eventual errors.
    Illegal,                    
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