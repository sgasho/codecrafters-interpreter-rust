pub trait PrjString {
    fn print_error(&self);
}

impl PrjString for String {
    fn print_error(&self) {
        eprintln!("{self}");
    }
}

pub trait PrjChar {
    fn is_identifier_char(&self) -> bool;
}

impl PrjChar for char {
    fn is_identifier_char(&self) -> bool {
        matches!(*self, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
    }
}