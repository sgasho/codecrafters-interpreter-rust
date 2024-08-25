pub trait Node {
    fn string(&self) -> String;
}

pub trait Statement: Node {}

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

impl Statement for Boolean {}

pub struct Nil {}

impl Node for Nil {
    fn string(&self) -> String {
        "nil".to_string()
    }
}

impl Statement for Nil {}