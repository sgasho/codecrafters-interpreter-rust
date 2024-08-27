pub enum ObjectType {
    BooleanObj,
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

pub struct Nil {}

impl Object for Nil {
    fn obj_type(&self) -> ObjectType {
        ObjectType::NilObj
    }
    fn inspect(&self) -> String {
        "nil".to_string()
    }
}