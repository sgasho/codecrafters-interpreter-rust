pub trait PrjString {
    fn print(&self);
}

impl PrjString for String {
    fn print(&self) {
        println!("{self}");
    }
}