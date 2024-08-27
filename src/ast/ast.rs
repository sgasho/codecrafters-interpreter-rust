use std::any::Any;
use crate::lexer::lexer::{Token};

#[derive(Debug)]
pub enum NodeType {
    Program,
    ExpressionStatement,
    Boolean,
    NumberLiteral,
    StringLiteral,
    Grouping,
    PrefixExpression,
    InfixExpression,
    Nil,
}

pub trait Node: Any {
    fn string(&self) -> String;
    fn node_type(&self) -> NodeType;
    fn as_any(&self) -> &dyn Any;
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
    fn node_type(&self) -> NodeType {
        NodeType::ExpressionStatement
    }

    fn as_any(&self) -> &dyn Any {
        self
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

impl Node for Program {
    fn string(&self) -> String {
        "program".to_string()
    }

    fn node_type(&self) -> NodeType {
        NodeType::Program
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Boolean {
    pub value: bool,
}

impl Node for Boolean {
    fn string(&self) -> String {
        self.value.to_string()
    }
    fn node_type(&self) -> NodeType {
        NodeType::Boolean
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for Boolean {}

pub struct Nil {}

impl Node for Nil {
    fn string(&self) -> String {
        "nil".to_string()
    }
    fn node_type(&self) -> NodeType {
        NodeType::Nil
    }

    fn as_any(&self) -> &dyn Any {
        self
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
    fn node_type(&self) -> NodeType {
        NodeType::NumberLiteral
    }

    fn as_any(&self) -> &dyn Any {
        self
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
    fn node_type(&self) -> NodeType {
        NodeType::StringLiteral
    }

    fn as_any(&self) -> &dyn Any {
        self
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
    fn node_type(&self) -> NodeType {
        NodeType::Grouping
    }

    fn as_any(&self) -> &dyn Any {
        self
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
    fn node_type(&self) -> NodeType {
        NodeType::PrefixExpression
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for PrefixExpression {}

pub struct InfixExpression {
    pub token: Token,
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Node for InfixExpression {
    fn string(&self) -> String {
        format!("({} {} {})", self.token.lexeme, self.left.string(), self.right.string())
    }
    fn node_type(&self) -> NodeType {
        NodeType::InfixExpression
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for InfixExpression {}