#[derive(Debug, PartialEq)]
pub enum TokenType {
    LParen,
    RParen,
    EOF,
    Nil,
}

pub struct Token {
    pub token_type: TokenType,
    lexem: String,
    literal: String,
}

impl Token {
    pub fn print(&self) {
        println!("{} {} {}", self.token_type_to_print(), self.lexem, self.literal);
    }

    fn token_type_to_print(&self) -> &str {
        match self.token_type {
            TokenType::LParen => "LEFT_PAREN",
            TokenType::RParen => "RIGHT_PAREN",
            TokenType::EOF => "EOF",
            TokenType::Nil => "Nil",
        }
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn read_char(&mut self) -> char {
        if self.position >= self.input.len() {
            return '\0';
        }
        let pos = self.position;
        self.position += 1;
        self.input[pos]
    }

    pub fn new_token(&mut self) -> Token {
        let mut t = TokenType::Nil;
        let mut lexem = "";
        let literal = "null";

        let ch = self.read_char();

        match ch {
            '(' => {
                t = TokenType::LParen;
                lexem = "(";
            }
            ')' => {
                t = TokenType::RParen;
                lexem = ")";
            }
            '\0' => {
                t = TokenType::EOF;
            }
            _ => {}
        }

        Token {
            token_type: t,
            lexem: lexem.to_string(),
            literal: literal.to_string(),
        }
    }
}
