use crate::ast::ast::{Expression, Node, Program, Statement};
use crate::object::object::{Boolean as BooleanObject, Nil as NilObject, Object};
use crate::ast::ast::{Boolean, Nil, ExpressionStatement};

pub fn eval(node: Box<dyn Node>) -> Box<dyn Object> {
    if let Some(p) = node.as_any().downcast_ref::<Program>() {
        for stmt in p.statements.iter() {
            return eval_statement(stmt);
        }
    }
    if let Some(b) = node.as_any().downcast_ref::<Boolean>() {
        Box::new(BooleanObject { value: b.value })
    } else if let Some(_) = node.as_any().downcast_ref::<Nil>() {
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
    Box::new(NilObject {})
}
