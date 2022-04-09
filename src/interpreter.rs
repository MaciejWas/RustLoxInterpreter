use std::io::Write;
use std::fmt;
use std::io;

use errors::LoxError;
use scanner::Scanner;

pub mod tokens;
pub mod scanner;
// pub mod parser;
pub mod errors;
pub mod text_reader;

fn  print_with_flush<T>(text: T)
where T: fmt::Display
{
    print!("{}", text);
    io::stdout().flush().expect("Flush failed!");
}

pub struct LoxInterpreter {}

impl LoxInterpreter {
    pub fn new() -> Self {
        LoxInterpreter {}
    }

    pub fn run_prompt(&self) {
        let mut buffer = String::new();
        let stdin = io::stdin();
        loop {
            print_with_flush(" Lox >>> ");
            let next_line = stdin.read_line(&mut buffer);

            match next_line {
                Ok(0) => {},
                Ok(_) => {
                    let response = self.run(buffer.clone());
                    match response {
                        Ok(text) => println!("{}", text),
                        Err(error_message) => {
                            println!("{}", error_message);
                            break
                        }
                    }
                },
                Err(_) => {}
            }
            buffer.clear();
        }
    }

    pub fn run_file(&self, path: &String) {}

    fn run(&self, statement: String) -> Result<String, LoxError> {
        let scanner_output = Scanner::new(statement).scan();
        let response = format!("{:?}", scanner_output?.tokens);
        Ok(response)
    }
}