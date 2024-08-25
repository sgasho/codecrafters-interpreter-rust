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

pub struct Number {
    pub value: f64,
}

impl Node for Number {
    fn string(&self) -> String {
        self.value.to_string()
    }
}

impl Expression for Number {}