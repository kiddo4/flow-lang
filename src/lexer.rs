use crate::error::{FlowError, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    String(String),
    Integer(i64),
    BigInteger(crate::bigint::BigInt),
    Float(f64),
    Boolean(bool),
    
    // Identifiers
    Identifier(String),
    
    // Keywords
    Let,
    Be,
    Def,
    With,
    Do,
    End,
    If,
    Then,
    Else,
    ElseIf,
    While,
    For,
    From,
    To,
    Show,
    Return,
    Import,
    Export,
    As,
    Try,
    Catch,
    True,
    False,
    
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
    Not,
    
    // Punctuation
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Colon,
    Arrow,
    Ellipsis,
    Assign,
    
    // Special
    Newline,
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        
        while !self.is_at_end() {
            self.skip_whitespace();
            
            if self.is_at_end() {
                break;
            }
            
            // Skip comments
            if self.current_char() == '#' {
                self.skip_comment();
                continue;
            }
            
            let token = self.next_token()?;
            tokens.push(token);
        }
        
        tokens.push(Token::Eof);
        Ok(tokens)
    }
    
    fn next_token(&mut self) -> Result<Token> {
        let ch = self.current_char();
        
        match ch {
            '\n' => {
                self.advance();
                self.line += 1;
                self.column = 1;
                Ok(Token::Newline)
            }
            '+' => {
                self.advance();
                Ok(Token::Plus)
            }
            '-' => {
                self.advance();
                Ok(Token::Minus)
            }
            '*' => {
                self.advance();
                Ok(Token::Multiply)
            }
            '/' => {
                self.advance();
                Ok(Token::Divide)
            }
            '%' => {
                self.advance();
                Ok(Token::Modulo)
            }
            '(' => {
                self.advance();
                Ok(Token::LeftParen)
            }
            ')' => {
                self.advance();
                Ok(Token::RightParen)
            }
            '[' => {
                self.advance();
                Ok(Token::LeftBracket)
            }
            ']' => {
                self.advance();
                Ok(Token::RightBracket)
            }
            '{' => {
                self.advance();
                Ok(Token::LeftBrace)
            }
            '}' => {
                self.advance();
                Ok(Token::RightBrace)
            }
            ',' => {
                self.advance();
                Ok(Token::Comma)
            }
            '.' => {
                if self.peek() == '.' && self.peek_next() == '.' {
                    self.advance(); // consume first '.'
                    self.advance(); // consume second '.'
                    self.advance(); // consume third '.'
                    Ok(Token::Ellipsis)
                } else {
                    self.advance();
                    Ok(Token::Dot)
                }
            }
            ':' => {
                self.advance();
                Ok(Token::Colon)
            }
            '=' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(Token::Equal)
                } else if self.current_char() == '>' {
                    self.advance();
                    Ok(Token::Arrow)
                } else {
                    Ok(Token::Assign)
                }
            }
            '!' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(Token::NotEqual)
                } else {
                    Err(FlowError::lexer_error(
                        self.line,
                        self.column,
                        "Unexpected character '!'. Did you mean '!='?",
                    ))
                }
            }
            '>' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(Token::GreaterEqual)
                } else {
                    Ok(Token::Greater)
                }
            }
            '<' => {
                self.advance();
                if self.current_char() == '=' {
                    self.advance();
                    Ok(Token::LessEqual)
                } else {
                    Ok(Token::Less)
                }
            }
            '"' => self.read_string(),
            ch if ch.is_ascii_digit() => self.read_number(),
            ch if ch.is_ascii_alphabetic() || ch == '_' => self.read_identifier(),
            _ => Err(FlowError::lexer_error(
                self.line,
                self.column,
                format!("Unexpected character: '{}'", ch),
            )),
        }
    }
    
    fn read_string(&mut self) -> Result<Token> {
        self.advance(); // Skip opening quote
        let mut value = String::new();
        
        while !self.is_at_end() && self.current_char() != '"' {
            if self.current_char() == '\\' {
                self.advance();
                if self.is_at_end() {
                    return Err(FlowError::lexer_error(
                        self.line,
                        self.column,
                        "Unterminated string literal",
                    ));
                }
                
                match self.current_char() {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    ch => {
                        return Err(FlowError::lexer_error(
                            self.line,
                            self.column,
                            format!("Invalid escape sequence: \\{}", ch),
                        ))
                    }
                }
            } else {
                value.push(self.current_char());
            }
            self.advance();
        }
        
        if self.is_at_end() {
            return Err(FlowError::lexer_error(
                self.line,
                self.column,
                "Unterminated string literal",
            ));
        }
        
        self.advance(); // Skip closing quote
        Ok(Token::String(value))
    }
    
    fn read_number(&mut self) -> Result<Token> {
        let mut value = String::new();
        let mut is_float = false;
        
        while !self.is_at_end() && (self.current_char().is_ascii_digit() || self.current_char() == '.') {
            if self.current_char() == '.' {
                if is_float {
                    break; // Second dot, stop parsing
                }
                is_float = true;
            }
            value.push(self.current_char());
            self.advance();
        }
        
        if is_float {
            match value.parse::<f64>() {
                Ok(f) => Ok(Token::Float(f)),
                Err(_) => Err(FlowError::lexer_error(
                    self.line,
                    self.column,
                    format!("Invalid float literal: {}", value),
                )),
            }
        } else {
            match value.parse::<i64>() {
                Ok(i) => Ok(Token::Integer(i)),
                Err(_) => {
                    // Try parsing as BigInt for large numbers
                    match crate::bigint::BigInt::from_string(&value) {
                        Ok(big_int) => Ok(Token::BigInteger(big_int)),
                        Err(_) => Err(FlowError::lexer_error(
                            self.line,
                            self.column,
                            format!("Invalid integer literal: {}", value),
                        )),
                    }
                }
            }
        }
    }
    
    fn read_identifier(&mut self) -> Result<Token> {
        let mut value = String::new();
        
        while !self.is_at_end() && (self.current_char().is_ascii_alphanumeric() || self.current_char() == '_') {
            value.push(self.current_char());
            self.advance();
        }
        
        let token = match value.as_str() {
            "let" => Token::Let,
            "be" => Token::Be,
            "def" => Token::Def,
            "with" => Token::With,
            "do" => Token::Do,
            "end" => Token::End,
            "if" => Token::If,
            "then" => Token::Then,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "from" => Token::From,
            "to" => Token::To,
            "show" => Token::Show,
            "return" => Token::Return,
            "import" => Token::Import,
            "export" => Token::Export,
            "as" => Token::As,
            "try" => Token::Try,
            "catch" => Token::Catch,
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            _ => Token::Identifier(value),
        };
        
        Ok(token)
    }
    
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.current_char().is_whitespace() && self.current_char() != '\n' {
            self.advance();
        }
    }
    
    fn skip_comment(&mut self) {
        while !self.is_at_end() && self.current_char() != '\n' {
            self.advance();
        }
    }
    
    fn current_char(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.position]
        }
    }
    
    fn advance(&mut self) {
        if !self.is_at_end() {
            self.position += 1;
            self.column += 1;
        }
    }
    
    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
    
    fn peek(&self) -> char {
        if self.position + 1 >= self.input.len() {
            '\0'
        } else {
            self.input[self.position + 1]
        }
    }
    
    fn peek_next(&self) -> char {
        if self.position + 2 >= self.input.len() {
            '\0'
        } else {
            self.input[self.position + 2]
        }
    }
}