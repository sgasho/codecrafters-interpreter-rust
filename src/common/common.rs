pub trait PrjString {
    fn print_error(&self);
}

impl PrjString for String {
    fn print_error(&self) {
        eprintln!("{self}");
    }
}

pub trait PrjChar {
    fn is_alphabet_or_underscore(&self) -> bool;
}

impl PrjChar for char {
    fn is_alphabet_or_underscore(&self) -> bool {
        matches!(*self, 'a'..='z' | 'A'..='Z' | '_')
    }
}