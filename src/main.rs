use std::env;

pub mod interpreter;
use interpreter::LoxInterpreter;

/// Handles arguments from the command line and calls appropiate methods from `LoxInterpreter`.
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = LoxInterpreter::new();
    match args.len() {
        1 => interpreter.run_prompt(),
        2 => interpreter.run_file(&args[1]).map_or((), |_| ()),
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(64)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::LoxInterpreter;

    #[test]
    fn test_print() {
        let mut interpreter = LoxInterpreter::new();
        let result = interpreter.run_file(&"./src/integration_tests/test_print.js".to_string());
        assert_eq!(result.is_none(), true)
    }

    #[test]
    fn test_expr_stmt() {
        let mut interpreter = LoxInterpreter::new();
        let result = interpreter.run_file(&"./src/integration_tests/test_expr_stmt.js".to_string());
        assert_eq!(result.is_none(), true)
    }

    #[test]
    fn test_assignments() {
        let mut interpreter = LoxInterpreter::new();
        let result =
            interpreter.run_file(&"./src/integration_tests/test_assignments.js".to_string());
        assert_eq!(result.is_none(), true)
    }

    #[test]
    fn test_functions() {
        let mut interpreter = LoxInterpreter::new();
        let result =
            interpreter.run_file(&"./src/integration_tests/test_function_defn.js".to_string());
        assert_eq!(result.is_none(), true)
    }

    #[test]
    fn test_functions_calls() {
        let mut interpreter = LoxInterpreter::new();
        let result = interpreter.run_file(&"./src/integration_tests/test_fn_call.js".to_string());
        assert_eq!(result.is_none(), true)
    }

    #[test]
    fn test_class() {
        let mut interpreter = LoxInterpreter::new();
        let result = interpreter.run_file(&"./src/integration_tests/test_class.js".to_string());
        assert_eq!(result.is_none(), true)
    }
}
