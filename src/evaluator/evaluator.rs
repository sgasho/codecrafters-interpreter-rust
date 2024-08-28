use crate::ast::ast::{Expression, Node, Program, Statement, StringLiteral};
use crate::object::object::{
    Boolean as BooleanObject,
    Nil as NilObject,
    NumberLiteral as NumberLiteralObject,
    StringLiteral as StringLiteralObject,
    Object
};
use crate::ast::ast::{Boolean, NumberLiteral, Nil, ExpressionStatement};

pub fn eval(node: Box<dyn Node>) -> Box<dyn Object> {
    if let Some(p) = node.as_any().downcast_ref::<Program>() {
        for stmt in p.statements.iter() {
            return eval_statement(stmt);
        }
    }
    if let Some(b) = node.as_any().downcast_ref::<Boolean>() {
        return Box::new(BooleanObject { value: b.value });
    }
    if let Some(n) = node.as_any().downcast_ref::<NumberLiteral>() {
        return Box::new(NumberLiteralObject { value: n.value, literal: n.literal.to_string() })
    }
    if let Some(s) = node.as_any().downcast_ref::<StringLiteral>() {
        return Box::new(StringLiteralObject { value: s.value.to_string() })
    }
    if let Some(_) = node.as_any().downcast_ref::<Nil>() {
        Box::new(NilObject {})
    } else {
        Box::new(NilObject {})
    }
}

fn eval_statement(stmt: &Box<dyn Statement>) -> Box<dyn Object> {
    if let Some(e) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
        return eval_expression(&e.expression);
    }
    Box::new(NilObject {})
}

fn eval_expression(exp: &Box<dyn Expression>) -> Box<dyn Object> {
    if let Some(b) = exp.as_any().downcast_ref::<Boolean>() {
        return Box::new(BooleanObject { value: b.value })
    }
    if let Some(n) = exp.as_any().downcast_ref::<NumberLiteral>() {
        return Box::new(NumberLiteralObject { value: n.value, literal: n.literal.to_string() })
    }
    if let Some(s) = exp.as_any().downcast_ref::<StringLiteral>() {
        return Box::new(StringLiteralObject { value: s.value.to_string() })
    }
    Box::new(NilObject {})
}
