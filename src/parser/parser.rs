use crate::ast::ast::{Boolean, Expression, ExpressionStatement, Nil, Number, Program, Statement};
use crate::lexer::lexer::{Lexer, Token, TokenType};
use crate::lexer::lexer::TokenType::EOF;

pub struct Parser {
    lexer: Lexer,
    off: usize,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            off: 0,
        }
    }

    fn current_token(&self) -> Option<&Token> {
        if self.off >= self.lexer.tokens.len() {
            return None;
        }
        Some(&self.lexer.tokens[self.off])
    }

    fn current_token_type(&self) -> Option<&TokenType> {
        match self.current_token() {
            Some(token) => Some(&token.token_type),
            None => None
        }
    }

    fn next_token(&mut self) {
        if self.off < self.lexer.tokens.len() {
            self.off += 1;
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: Vec::new() };
        while let Some(token) = self.current_token() {
            if token.token_type == EOF {
                break;
            }

            let stmt = self.parse_statement();
            program.statements.push(stmt);
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Box<dyn Statement> {
        match self.current_token_type() {
            _ => Box::new(self.parse_expression_statement()),
        }
    }

    fn parse_expression_statement(&mut self) -> ExpressionStatement {
        ExpressionStatement {
            expression: self.parse_expression(),
        }
    }

    fn parse_expression(&mut self) -> Box<dyn Expression> {
        match self.current_token_type() {
            Some(TokenType::True) => {
                match self.current_token().cloned() {
                    Some(_) => Box::new(Boolean {value: true}),
                    None => Box::new(Nil {}),
                }
            }
            Some(TokenType::False) => {
                match self.current_token().cloned() {
                    Some(_) => Box::new(Boolean {value: false}),
                    None => Box::new(Nil {}),
                }
            }
            Some(TokenType::Number) => {
                match self.current_token().cloned() {
                    Some(token) => {
                        Box::new(Number { value: token.lexeme.parse().unwrap()})
                    }
                    None => Box::new(Nil {}),
                }
            }
            _ => Box::new(Nil{})
        }
    }
}