use std::fmt;

/// Represents the different types of tokens in the language.
/// These are used by the lexer to categorize pieces of input.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    /// A keyword like `def`, `header`, etc.
    Keyword(String),        

    /// An identifier such as variables names or tag names
    Identifier(String),     

    /// A String literal enclosed in double quotes
    StringLiteral(String),   

    /// A numeric literal, including integers and floats.
    Number(f64),            

    /// Symbols and punctuation
    Equals,                 // '='
    Comma,                  // ','
    Semicolon,              // ';'
    LParen, RParen,         // '(' and ')'
    LBrace, RBrace,         // '{' and '}'
    
    /// Comparison operators
    Not,                    // '!'
    EqualsTo,               // '=='    
    NotEqualsTo,            // '!='
    LessThan, GreaterThan,  // '<' and '>'
    LessOrEqual,            // '<='
    GreaterOrEqual,         // '>='

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
            TokenType::Keyword(_)       => "Keyword",
            TokenType::Identifier(_)    => "Identifier",
            TokenType::StringLiteral(_) => "StringLiteral",
            TokenType::Number(_)        => "Number",
            TokenType::Equals           => "Equals",
            TokenType::Comma            => "Comma",
            TokenType::Semicolon        => "Semicolon",
            TokenType::LParen           => "LParen",
            TokenType::RParen           => "RParen",
            TokenType::LBrace           => "LBrace",
            TokenType::RBrace           => "RBrace",
            TokenType::Not              => "Not",
            TokenType::EqualsTo         => "EqualsTo",
            TokenType::NotEqualsTo      => "NotEqualsTo",
            TokenType::LessThan         => "LessThan",
            TokenType::GreaterThan      => "GreaterThan",
            TokenType::LessOrEqual      => "LessOrEqual",
            TokenType::GreaterOrEqual   => "GreaterOrEqual",
            TokenType::EOF              => "EOF",
            TokenType::Illegal          => "Illegal",
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
pub struct Token {
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