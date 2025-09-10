use std::fmt;

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

impl fmt::Display for TokenType {
    /// Formats the token type as a more readable String.
    /// 
    /// This is used for diagnostics, logging, and pretty-printing.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            TokenType::Keyword      => "Keyword",
            TokenType::Identifier   => "Identifier",
            TokenType::StringLiteral=> "StringLiteral",
            TokenType::Number       => "Number",
            TokenType::Equals       => "Equals",
            TokenType::Comma        => "Comma",
            TokenType::Semicolon    => "Semicolon",
            TokenType::LParen       => "LParen",
            TokenType::RParen       => "RParen",
            TokenType::LBrace       => "LBrace",
            TokenType::RBrace       => "RBrace",
            TokenType::LessThan     => "LessThan",
            TokenType::GreaterThan  => "GreaterThan",
            TokenType::EOF          => "EOF",
            TokenType::Illegal      => "Illegal",
        };
        write!(f, "{}", name)
    }
}

/// Represents a single token extracted from the source code.
/// 
/// Each token includes: 
/// - `value`: the raw String from the input
/// - `token_type`: its syntactic category
/// - `line`: the line number where the token appears (starting at 1)
/// - `column`: the column number where the token starts (starting at 1)
#[derive(Debug, Clone, PartialEq)]
struct Token {
    pub value: String,
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Token {
    /// Formats the token as a more readable String.
    /// 
    /// Example output: `[Keyword] 'def' at line 1, column 5`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] '{}' at line {}, column {}",
            self.token_type, self.value, self.line, self.column
        )
    }
}

/// The Lexer struct is responsible for scanning the input string
/// and producing a stream of tokens for the parser to consume.
struct Lexer {
    input: String,                  // The input source code as String
    position: usize,                // The current position in the input
    current_char: Option<char>,     // The character that is currently being examined
}