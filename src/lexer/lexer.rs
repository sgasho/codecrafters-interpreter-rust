#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Asterisk,
    Slash,
    EOF,
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    lexeme: &'static str,
    literal: &'static str,
}

impl Token {
    pub fn print(&self) {
        println!("{} {} {}", self.token_type_to_print(), self.lexeme, self.literal);
    }

    fn token_type_to_print(&self) -> &str {
        match self.token_type {
            TokenType::LParen => "LEFT_PAREN",
            TokenType::RParen => "RIGHT_PAREN",
            TokenType::LBrace => "LEFT_BRACE",
            TokenType::RBrace => "RIGHT_BRACE",
            TokenType::Comma => "COMMA",
            TokenType::Dot => "DOT",
            TokenType::Minus => "MINUS",
            TokenType::Plus => "PLUS",
            TokenType::Semicolon => "SEMICOLON",
            TokenType::Asterisk => "STAR",
            TokenType::Slash => "SLASH",
            TokenType::EOF => "EOF",
        }
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    pub tokens: Vec<Token>,
    pub errors: Vec<String>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            tokens: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn read_char(&mut self) -> Option<char> {
        if self.position >= self.input.len() {
            return None;
        }
        let pos = self.position;
        self.position += 1;
        Some(self.input[pos])
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: &'static str, literal: &'static str) {
        self.tokens.push(Token {
            token_type,
            lexeme,
            literal,
        });
    }
    pub fn tokenize(&mut self) {
        let mut line_number = 1;

        while let Some(ch) = self.read_char() {
            match ch {
                '(' => self.add_token(TokenType::LParen, "(", "null"),
                ')' => self.add_token(TokenType::RParen, ")", "null"),
                '{' => self.add_token(TokenType::LBrace, "{", "null"),
                '}' => self.add_token(TokenType::RBrace, "}", "null"),
                ',' => self.add_token(TokenType::Comma, ",", "null"),
                '.' => self.add_token(TokenType::Dot, ".", "null"),
                '-' => self.add_token(TokenType::Minus, "-", "null"),
                '+' => self.add_token(TokenType::Plus, "+", "null"),
                ';' => self.add_token(TokenType::Semicolon, ";", "null"),
                '*' => self.add_token(TokenType::Asterisk, "*", "null"),
                '/' => self.add_token(TokenType::Slash, "/", "null"),
                '\n' | '\r' => {
                    line_number += 1;
                    continue;
                }
                _ => {
                    self.errors.push(format!("[line {line_number}] Error: Unexpected character: {ch}"));
                    continue;
                }
            }
        }

        self.add_token(TokenType::EOF, "", "null");
    }
}
