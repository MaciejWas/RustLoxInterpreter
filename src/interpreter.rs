use crate::interpreter::execute::Executor;
use crate::interpreter::parser::visitor::Visitor;
use std::fmt;
use std::io;
use std::io::Write;

use errors::LoxError;
use parser::{Parser};
use scanner::Scanner;

pub mod errors;
pub mod execute;
pub mod parser;
pub mod readers;
pub mod scanner;
pub mod tokens;

fn print_with_flush<T>(text: T)
where
    T: fmt::Display,
{
    print!("{}", text);
    io::stdout().flush().expect("Flush failed!");
}

pub struct LoxInterpreter {
    executor: Executor,
}

impl LoxInterpreter {
    pub fn new() -> Self {
        LoxInterpreter {
            executor: Executor::new(),
        }
    }

    pub fn run_prompt(&mut self) {
        let mut buffer = String::new();
        let stdin = io::stdin();
        loop {
            print_with_flush(" Lox >>> ");
            let next_line = stdin.read_line(&mut buffer);

            match next_line {
                Ok(0) => {}
                Ok(_) => self.interpret_line_and_respond(buffer.clone()),
                Err(err) => {
                    self.handle_err(&err);
                    break;
                }
            };
            buffer.clear();
        }
    }

    fn interpret_line_and_respond(&mut self, mut line: String) {
        line = line.replace("\\n", "\n");
        line = line.trim().to_string();

        if line.ends_with('\r') {}

        let response = self.run(line.clone());
        match response {
            Ok(_) => {}
            Err(error_message) => {
                println!("{}", error_message.generate_err_msg(&line));
            }
        }
    }

    pub fn run_file(&mut self, path: &String) {
        let content = std::fs::read_to_string(path).expect("Something went wrong reading the file");

        let scanner_output = Scanner::new(content.clone()).scan();
        if let Err(err) = scanner_output {
            println!("{}", err.generate_err_msg(&content));
            return ();
        }

        let parser_output = Parser::new(scanner_output.unwrap()).parse();
        if let Err(err) = parser_output {
            println!("{}", err.generate_err_msg(&content));
            return ();
        }

        let executor_output = self.executor.visit(&parser_output.unwrap());
        if let Err(err) = executor_output {
            println!("{}", err.generate_err_msg(&content));
            return ();
        }
    }

    pub fn handle_err(&self, _err: &std::io::Error) {}

    fn run(&mut self, statement: String) -> Result<String, LoxError> {
        let scanner_output = Scanner::new(statement.clone()).scan()?;
        let parser_output = Parser::new(scanner_output).parse()?;
        self.executor.visit(&parser_output)?;

        Ok(format!("Ok!"))
    }
}
