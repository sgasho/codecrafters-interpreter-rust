pub trait PrjString {
    fn print_error(&self);
}

impl PrjString for String {
    fn print_error(&self) {
        eprintln!("{self}");
    }
}