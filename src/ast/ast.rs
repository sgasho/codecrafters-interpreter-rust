use crate::lexer::lexer::{Token, TokenType};

pub trait Node {
    fn string(&self) -> String;
}

pub trait Statement: Node {}

pub trait Expression: Node {}

pub struct ExpressionStatement {
    pub expression: Box<dyn Expression>,
}

impl Node for ExpressionStatement {
    fn string(&self) -> String {
        self.expression.string()
    }
}

impl Statement for ExpressionStatement {}
impl Expression for ExpressionStatement {}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn print(&self) {
        for stmt in self.statements.iter() {
            println!("{}", stmt.string());
        }
    }
}

pub struct Boolean {
    pub value: bool,
}

impl Node for Boolean {
    fn string(&self) -> String {
        self.value.to_string()
    }
}

impl Expression for Boolean {}

pub struct Nil {}

impl Node for Nil {
    fn string(&self) -> String {
        "nil".to_string()
    }
}

impl Expression for Nil {}

pub struct NumberLiteral {
    pub value: f64,
    pub literal: String,
}

impl Node for NumberLiteral {
    fn string(&self) -> String {
        self.literal.to_string()
    }
}

impl Expression for NumberLiteral {}

pub struct StringLiteral {
    pub value: String,
}

impl Node for StringLiteral {
    fn string(&self) -> String {
        self.value.to_string()
    }
}

impl Expression for StringLiteral {}

pub struct Grouping {
    pub expression: Box<dyn Expression>,
}

impl Node for Grouping {
    fn string(&self) -> String {
        format!("(group {})", self.expression.string())
    }
}

impl Expression for Grouping {}

pub struct PrefixExpression {
    pub operator: Token,
    pub right: Box<dyn Expression>,
}

impl Node for PrefixExpression {
    fn string(&self) -> String {
        format!("({} {})", self.operator.lexeme, self.right.string())
    }
}

impl Expression for PrefixExpression {}