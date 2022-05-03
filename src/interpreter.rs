use std::io::Write;
use std::fmt;
use std::io;

use errors::LoxError;
use scanner::Scanner;
use parser::{
    Parser,
    pretty_printing::PrettyPrint,
    evaluating::Evaluate
};


pub mod tokens;
pub mod scanner;
pub mod parser;
pub mod errors;
pub mod readers;


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
                Ok(_) => self.interpret_line_and_respond(buffer.clone()),
                Err(err) => {self.handle_err(&err); break}
            };
            buffer.clear();
        }
    }

    fn interpret_line_and_respond(&self, line: String) {
        let response = self.run(line.clone());
        match response {
            Ok(text) => println!("{}", text),
            Err(error_message) => {
                println!("{}", error_message.generate_err_msg(&line));
            }
        }
    }

    pub fn run_file(&self, path: &String) {}

    pub fn handle_err(&self, err: &std::io::Error) {}

    fn run(&self, statement: String) -> Result<String, LoxError> {
        let scanner_output = Scanner::new(statement.clone())
                                     .scan()?;
        let parser_output = Parser::new(scanner_output)
                                   .parse()?;
    
        let response = format!("{:?}", parser_output.pretty_print(0));
        println!("Evaluated: {:?}", parser_output.eval());
        Ok(response)
    }
}
