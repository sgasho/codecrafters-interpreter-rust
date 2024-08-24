#[derive(Debug, PartialEq)]
pub enum TokenType {
    LParen,
    RParen,
    LBrace,
    RBrace,
    EOF,
    Nil,
}

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
        let mut lexeme = "";
        let literal = "null";

        let ch = self.read_char();

        match ch {
            '(' => {
                t = TokenType::LParen;
                lexeme = "(";
            }
            ')' => {
                t = TokenType::RParen;
                lexeme = ")";
            }
            '{' => {
                t = TokenType::LBrace;
                lexeme = "{";
            }
            '}' => {
                t = TokenType::RBrace;
                lexeme = "}";
            }
            '\0' => {
                t = TokenType::EOF;
            }
            _ => {}
        }

        Token {
            token_type: t,
            lexeme,
            literal,
        }
    }
}
