use crate::token::{Token, TokenType};

/// The Lexer struct is responsible for scanning the input string
/// and producing a stream of tokens for the parser to consume.
struct Lexer {
    input: Vec<char>,               // Convert input to a char vector for better indexing
    position: usize,                // Current index in the input
    line: usize,                    // Line number (starts at 1)
    column: usize,                  // Column number (starts at 1) 
}

impl Lexer {
    /// Create a new Lexer from the given source String.
    pub fn new(source: &str) -> Self {
        Lexer{
            input: source.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Return the current character without advancing.
    fn peek_current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    /// Return the next character without advancing.
    fn peek_next_char(&self) -> Option<char> {
        // TODO: Implement peek_next_char for multi-character tokens like '=='
        self.input.get(self.position + 1).copied()
    }

    /// Advance to the next character and updates line/column.
    fn advance(&mut self) -> Option<char> {
        let ch = self.peek_current_char();
        if let Some(c) = ch {
            self.position += 1;
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }

        ch
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;

        while let Some(ch) = self.peek_current_char() {
            if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                self.advance();
            } else {
                break;
            }
        }

        self.input[start..self.position].iter().collect()
    }

    fn read_number(&mut self) -> String {
        let start = self.position;
        let mut seen_dot = false;
        while let Some(ch) = self.peek_current_char() {
            if ch.is_ascii_digit() {
                self.advance();
            } else if ch == '.' && !seen_dot {
              seen_dot = true;
              self.advance();
            } else {
                break;
            }
        }

        self.input[start..self.position].iter().collect()
    }

    /// Produce the next token from the input stream.
    pub fn next_token(&mut self) -> Token{
        while let Some(ch) = self.peek_current_char() {
            // Skip whitespace
            if ch.is_whitespace() {
                self.advance(); 
                continue;
            }

            let line = self.line;
            let column = self.column;

            // Identifiers and Keywords
            if ch.is_alphabetic() || ch == '_' {
                let ident = self.read_identifier();

                let token_type = match ident.as_str() {
                    "def" | "header" | "body" => TokenType::Keyword(ident.clone()),
                    _ => TokenType::Identifier(ident.clone()),
                };

                return Token { 
                    value: ident, 
                    token_type, 
                    line, 
                    column 
                };
            }

            if ch.is_ascii_digit() {
                let num_str = self.read_number();
                let num_value = num_str.parse::<f64>().unwrap_or(0.0);

                return Token {
                    value: num_str,
                    token_type: TokenType::Number(num_value),
                    line,
                    column,
                };
            }
            
            let token_type = match ch {
                '=' => TokenType::Equals,
                ',' => TokenType::Comma,
                ';' => TokenType::Semicolon,
                '(' => TokenType::LParen,
                ')' => TokenType::RParen,
                '{' => TokenType::LBrace,
                '}' => TokenType::RBrace,
                '<' => TokenType::LessThan,
                '>' => TokenType::GreaterThan,
                _ => TokenType::Illegal,
            };

            let value = ch.to_string();
            self.advance();

            return Token {
                value, 
                token_type, 
                line, 
                column,
            }
        }   

        // End of input
        Token {
            value: "".to_string(),
            token_type: TokenType::EOF,
            line: self.line,
            column: self.column,
        } 
    }

    /// Returns true if lexer reached the end of input stream.
    fn is_at_end(&self) -> bool {
        // TODO: Use is_at_end() in loops or parser logic
        self.position >= self.input.len()
    }

}

/// Tokenizes the given source code String into a vector of tokens.
/// 
/// This function scans the input and returns a list of `Token`
/// instances, each representing a meaningful unit in the language.
pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(source_code);

    loop {
        let token = lexer.next_token();
        tokens.push(token.clone());

        if token.token_type == TokenType::EOF {
            break;
        }
    }

    tokens
}