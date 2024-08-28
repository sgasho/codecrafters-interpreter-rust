pub enum ObjectType {
    BooleanObj,
    StringLiteralObj,
    NumberLiteralObj,
    NilObj,
}

pub trait Object {
    fn obj_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}

pub struct Boolean {
    pub value: bool,
}

impl Object for Boolean {
    fn obj_type(&self) -> ObjectType {
        ObjectType::BooleanObj
    }
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

pub struct StringLiteral {
    pub value: String,
}

impl Object for StringLiteral {
    fn obj_type(&self) -> ObjectType {
        ObjectType::StringLiteralObj
    }
    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

pub struct NumberLiteral {
    pub value: f64,
    pub literal: String,
}

impl Object for NumberLiteral {
    fn obj_type(&self) -> ObjectType {
        ObjectType::NumberLiteralObj
    }
    fn inspect(&self) -> String {
        self.literal.to_string()
    }
}

pub struct Nil {}

impl Object for Nil {
    fn obj_type(&self) -> ObjectType {
        ObjectType::NilObj
    }
    fn inspect(&self) -> String {
        "nil".to_string()
    }
}