mod lexer;
mod common;
mod parser;
mod ast;

use std::{env, process};
use std::fs;
use std::io::{self, Write};
use crate::common::common::PrjString;
use crate::parser::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let mut l = lexer::lexer::Lexer::new(file_contents);
                l.tokenize();

                for err in l.errors.iter() {
                    err.print_error();
                }

                for token in l.tokens.iter() {
                    token.print();
                }

                if l.errors.len() > 0 {
                    process::exit(65);
                }
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        "parse" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let mut l = lexer::lexer::Lexer::new(file_contents);
                l.tokenize();

                if l.errors.len() > 0 {
                    process::exit(65);
                }

                let mut p = Parser::new(l);
                let pg = p.parse_program();

                for err in p.errors.iter() {
                   err.print_error();
                }

                if p.errors.len() > 0 {
                    process::exit(65);
                }

                pg.print();
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
