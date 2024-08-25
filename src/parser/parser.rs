use crate::ast::ast::{Boolean, Expression, ExpressionStatement, Grouping, InfixExpression, Nil, NumberLiteral, PrefixExpression, Program, Statement, StringLiteral};
use crate::lexer::lexer::{Lexer, Token, TokenType};
use crate::lexer::lexer::TokenType::{Asterisk, Slash, Plus, Minus, Less, LessEqual, Greater, GreaterEqual, EOF, Equal, NotEqual};

pub struct Parser {
    lexer: Lexer,
    off: usize,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            off: 0,
            errors: Vec::new(),
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

    fn current_token_line_number(&self) -> i32 {
        match self.current_token() {
            Some(token) => token.line_number,
            None => 0,
        }
    }

    fn next_token(&mut self) {
        if self.off < self.lexer.tokens.len() {
            self.off += 1;
        }
    }

    fn peek_token(&self) -> Option<&Token> {
        if self.off + 1 >= self.lexer.tokens.len() {
            return None;
        }
        Some(&self.lexer.tokens[self.off+1])
    }

    fn peek_token_type(&self) -> Option<&TokenType> {
        match self.peek_token() {
            Some(token) => Some(&token.token_type),
            None => None
        }
    }

    fn peek_token_type_is(&self, target: TokenType) -> bool {
        match self.peek_token_type() {
            Some(token_type) => token_type.clone() == target,
            None => false,
        }
    }

    fn peek_precedence(&self) -> i32 {
        match self.peek_token_type() {
            Some(token_type) => token_type.clone().precedence(),
            None => 0,
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
            expression: self.parse_expression(0),
        }
    }

    fn parse_expression(&mut self, precedence: i32) -> Box<dyn Expression> {
        let prefix = match self.current_token_type() {
            Some(TokenType::True | TokenType::False) => self.parse_boolean_expression(),
            Some(TokenType::Number) => self.parse_number_expression(),
            Some(TokenType::String) => self.parse_string_expression(),
            Some(TokenType::LParen) => self.parse_grouping_expression(),
            Some(TokenType::Bang | TokenType::Minus) => self.parse_prefix_expression(),
            _ => self.parse_nil_expression(),
        };
        let mut left = prefix;

        while precedence < self.peek_precedence() {
            match self.peek_token_type() {
                Some(Asterisk | Slash | Plus | Minus | Less | LessEqual | Greater | GreaterEqual | Equal | NotEqual) => {
                    self.next_token();
                    left = self.parse_infix_expression(left);
                }
                _ => {
                    return left;
                }
            }
        }

        left
    }

    fn parse_boolean_expression(&mut self) -> Box<dyn Expression> {
        Box::new(Boolean {
            value: match self.current_token_type() {
                Some(TokenType::True) => true,
                _ => false,
            }
        })
    }

    fn parse_number_expression(&mut self) -> Box<dyn Expression> {
        match self.current_token().cloned() {
            Some(token) => {
                Box::new(NumberLiteral { value: token.literal.parse().unwrap(), literal: token.literal})
            }
            None => Box::new(Nil {}),
        }
    }

    fn parse_string_expression(&mut self) -> Box<dyn Expression> {
        match self.current_token().cloned() {
            Some(token) => {
                Box::new(StringLiteral { value: token.literal })
            }
            None => Box::new(Nil {}),
        }
    }

    fn parse_nil_expression(&mut self) -> Box<dyn Expression> {
        Box::new(Nil {})
    }

    fn parse_grouping_expression(&mut self) -> Box<dyn Expression> {
        self.next_token();
        let exp = self.parse_expression(0);

        if !self.peek_token_type_is(TokenType::RParen) {
            self.errors.push(format!("[line {}] Expect ')'", self.current_token_line_number()));
            return self.parse_nil_expression();
        }

        self.next_token();
        Box::new(Grouping {
            expression: exp,
        })
    }

    fn parse_prefix_expression(&mut self) -> Box<dyn Expression> {
        match self.current_token().cloned() {
            Some(token) => {
                self.next_token();
                let right = self.parse_expression(6);
                Box::new( PrefixExpression {
                    operator: token.clone(),
                    right,
                } )
            }
            None => self.parse_nil_expression()
        }
    }

    fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Box<dyn Expression> {
        match self.current_token().cloned() {
            Some(token) => {
                let precedence = token.token_type.precedence();
                self.next_token();
                let right = self.parse_expression(precedence);
                Box::new(InfixExpression {
                    token,
                    left,
                    right,
                })
            },
            None => self.parse_nil_expression(),
        }
    }
}