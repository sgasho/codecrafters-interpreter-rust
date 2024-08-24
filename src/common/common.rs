use std::io::{self, Write};

pub trait PrjString {
    fn print_error(&self);
}

impl PrjString for String {
    fn print_error(&self) {
        let stderr = io::stderr();
        let mut handle = stderr.lock();
        writeln!(handle, "{}", self).unwrap();
    }
}