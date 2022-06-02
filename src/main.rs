use std::env;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod interpreter;
use interpreter::LoxInterpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = LoxInterpreter::new();
    match args.len() {
        1 => interpreter.run_prompt(),
        2 => interpreter.run_file(&args[0]),
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(64)
        }
    }
}
