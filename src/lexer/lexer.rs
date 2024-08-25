use std::collections::HashMap;
use crate::common::common::PrjChar;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
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
    Assign,
    Equal,
    Bang,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    String,
    Number,
    Identifier,
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    EOF,
}

impl TokenType {
    pub fn precedence(&self) -> i32 {
        match self {
            TokenType::LParen => 0,
            TokenType::Equal | TokenType::NotEqual => 1,
            TokenType::Less | TokenType::LessEqual | TokenType::Greater | TokenType::GreaterEqual => 2,
            TokenType::Plus | TokenType::Minus => 3,
            TokenType::Asterisk | TokenType::Slash => 4,
            _ => 0
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
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
            TokenType::Assign => "EQUAL",
            TokenType::Equal => "EQUAL_EQUAL",
            TokenType::Bang => "BANG",
            TokenType::NotEqual => "BANG_EQUAL",
            TokenType::Less => "LESS",
            TokenType::LessEqual => "LESS_EQUAL",
            TokenType::Greater => "GREATER",
            TokenType::GreaterEqual => "GREATER_EQUAL",
            TokenType::String => "STRING",
            TokenType::Number => "NUMBER",
            TokenType::Identifier => "IDENTIFIER",
            TokenType::And => "AND",
            TokenType::Class => "CLASS",
            TokenType::Else => "ELSE",
            TokenType::False => "FALSE",
            TokenType::For => "FOR",
            TokenType::Fun => "FUN",
            TokenType::If => "IF",
            TokenType::Nil => "NIL",
            TokenType::Or => "OR",
            TokenType::Print => "PRINT",
            TokenType::Return => "RETURN",
            TokenType::Super => "SUPER",
            TokenType::This => "THIS",
            TokenType::True => "TRUE",
            TokenType::Var => "VAR",
            TokenType::While => "WHILE",
            TokenType::EOF => "EOF",
        }
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    pub tokens: Vec<Token>,
    pub errors: Vec<String>,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input: input.chars().collect(),
            position: 0,
            tokens: Vec::new(),
            errors: Vec::new(),
            keywords: HashMap::new(),
        };

        lexer.init_keywords();
        lexer
    }

    fn init_keywords(&mut self) {
        self.keywords.insert("and".to_string(), TokenType::And);
        self.keywords.insert("class".to_string(), TokenType::Class);
        self.keywords.insert("else".to_string(), TokenType::Else);
        self.keywords.insert("false".to_string(), TokenType::False);
        self.keywords.insert("for".to_string(), TokenType::For);
        self.keywords.insert("fun".to_string(), TokenType::Fun);
        self.keywords.insert("if".to_string(), TokenType::If);
        self.keywords.insert("nil".to_string(), TokenType::Nil);
        self.keywords.insert("or".to_string(), TokenType::Or);
        self.keywords.insert("print".to_string(), TokenType::Print);
        self.keywords.insert("return".to_string(), TokenType::Return);
        self.keywords.insert("super".to_string(), TokenType::Super);
        self.keywords.insert("this".to_string(), TokenType::This);
        self.keywords.insert("true".to_string(), TokenType::True);
        self.keywords.insert("var".to_string(), TokenType::Var);
        self.keywords.insert("while".to_string(), TokenType::While);
    }

    pub fn read_char(&mut self) -> Option<char> {
        if self.position >= self.input.len() {
            return None;
        }
        let pos = self.position;
        self.position += 1;
        Some(self.input[pos])
    }

    fn expect_current_token(&mut self, ch: char) -> bool {
        if self.position >= self.input.len() {
            return false;
        }
        if self.input[self.position] == ch {
            self.position += 1;
            return true;
        }
        false
    }

    fn current_token_is(&self, ch: char) -> bool {
        if self.position >= self.input.len() {
            return false;
        }
        self.input[self.position] == ch
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: &'static str, literal: &'static str) {
        self.tokens.push(Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal: literal.to_string(),
        });
    }

    fn add_token_string(&mut self, token_type: TokenType, lexeme: String, literal: String) {
        self.tokens.push(Token {
            token_type,
            lexeme,
            literal,
        });
    }

    fn skip_line(&mut self) {
        while self.position < self.input.len() && !self.current_token_is('\n') && !self.current_token_is('\0') {
            self.position += 1;
        }
    }

    fn read_string(&mut self) -> Option<String> {
        let mut str= String::new();
        while self.input[self.position] != '"' {
            let ch = self.input[self.position];
            str.push(ch);
            self.position += 1;
            if self.position >= self.input.len() || self.current_token_is('\n') || self.current_token_is('\0') {
                return None;
            }
        }
        self.position += 1;
        Some(str)
    }

    // returns (lexeme, literal)
    fn read_number_str(&mut self, first: char) -> (String, String) {
        let mut literal = String::new();
        let mut lexeme = String::new();
        literal.push(first);
        lexeme.push(first);
        while self.is_char_ascii_digit() {
            literal.push(self.input[self.position]);
            lexeme.push(self.input[self.position]);
            self.position += 1;
        }
        if self.position < self.input.len() && self.input[self.position] == '.' {
            literal.push('.');
            lexeme.push('.');
            self.position += 1;
            while self.is_char_ascii_digit() {
                literal.push(self.input[self.position]);
                lexeme.push(self.input[self.position]);
                self.position += 1;
            }
            while literal.ends_with("0") {
                literal = literal.trim_end_matches('0').to_string();
            }
            if literal.ends_with(".") {
                literal.push_str("0");
            }
        } else {
            literal.push_str(".0");
        }
        return (lexeme, literal);
    }

    fn add_token_identifier(&mut self, ident: String) {
        if let Some(token_type) = self.keywords.get(&ident) {
            self.add_token_string(token_type.clone(), ident, "null".to_string());
        } else {
            self.add_token_string(TokenType::Identifier, ident, "null".to_string());
        }
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut ident = String::new();
        ident.push(first);
        while self.is_char_identifier_one() {
            ident.push(self.input[self.position]);
            self.position += 1;
        }
        ident
    }

    fn is_char_ascii_digit(&self) -> bool {
        self.position < self.input.len() && self.input[self.position].is_ascii_digit()
    }

    fn is_char_identifier_one(&self) -> bool {
        self.position < self.input.len() && self.input[self.position].is_identifier_char()
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
                '/' => {
                    if self.expect_current_token('/') {
                        self.skip_line();
                    } else {
                        self.add_token(TokenType::Slash, "/", "null")
                    }
                }
                '=' => {
                    if self.expect_current_token('=') {
                        self.add_token(TokenType::Equal, "==", "null");
                    } else {
                        self.add_token(TokenType::Assign, "=", "null");
                    }
                }
                '!' => {
                    if self.expect_current_token('=') {
                        self.add_token(TokenType::NotEqual, "!=", "null");
                    } else {
                        self.add_token(TokenType::Bang, "!", "null");
                    }
                }
                '<' => {
                    if self.expect_current_token('=') {
                        self.add_token(TokenType::LessEqual, "<=", "null");
                    } else {
                        self.add_token(TokenType::Less, "<", "null");
                    }
                }
                '>' => {
                    if self.expect_current_token('=') {
                        self.add_token(TokenType::GreaterEqual, ">=", "null");
                    } else {
                        self.add_token(TokenType::Greater, ">", "null");
                    }
                }
                '\n' | '\r' => {
                    line_number += 1;
                    continue;
                }
                '"' => match self.read_string() {
                        Some(str) =>  self.add_token_string(TokenType::String, format!("\"{}\"", str.clone()), str.clone()),
                        None => self.errors.push(format!("[line {line_number}] Error: Unterminated string.")),
                }
                '0'..='9' => {
                    let (lexeme, literal) = self.read_number_str(ch);
                    self.add_token_string(TokenType::Number, lexeme, literal);
                }
                _ if ch.is_identifier_char() => {
                    let ident = self.read_identifier(ch);
                    self.add_token_identifier(ident);
                }
                _ if ch.is_whitespace() => continue,
                _ => {
                    self.errors.push(format!("[line {line_number}] Error: Unexpected character: {ch}"));
                    continue;
                }
            }
        }

        self.add_token(TokenType::EOF, "", "null");
    }
}
