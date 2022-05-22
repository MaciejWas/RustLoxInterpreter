use std::cmp::{max, min};

#[derive(Debug)]
pub enum ErrType {
    ParsingErr,
    EvalErr,
    TokenizingErr,
    ScanningErr,
    LogicError,
    InterpreterError,
}

#[derive(Debug)]
pub struct LoxError {
    pub msg: String,
    pub err_type: ErrType,
    pub pos: usize,
}

impl LoxError {
    pub fn generate_err_msg(&self, program: &String) -> String {
        let start = max(self.pos as i32 - 10, 0) as usize;
        let end = min(self.pos + 10, program.len());

        let local_pos = self.pos - start;

        if start > end {
            panic!(
                "Failed to generate error message. Could not take slice [{}, {}] from {}",
                start, end, program
            );
        }

        let prelude: String = program[start..end].trim().to_string();
        let pointer: String = "-".to_string().repeat(local_pos) + "^";

        [prelude, pointer, self.msg.clone()].join("\n")
    }

    pub fn new_err<A>(msg: String, pos: usize, err_type: ErrType) -> LoxResult<A> {
        Err(Self {
            msg: msg,
            err_type: err_type,
            pos: pos,
        })
    }
}

pub type LoxResult<A> = Result<A, LoxError>;

#[cfg(test)]
mod tests {
    use super::ErrType;
    use super::LoxError;
    use quickcheck::quickcheck;

    #[test]
    fn test_err_msg() {
        let msg: &str = "Test message!";
        let e: LoxError = LoxError {
            msg: msg.to_string(),
            err_type: ErrType::TokenizingErr,
            pos: 0,
        };
        assert_eq!(
            e.generate_err_msg(&"XXXXXXXXXXXXXXXXX".to_string()),
            "XXXXXXXXXX\n".to_string() + msg
        );
    }

    quickcheck! {
      fn quickcheck_err_construct(msg: String, pos: usize) -> bool {
          LoxError {msg: msg, err_type: ErrType::TokenizingErr, pos: pos};
          true
        }
    }
}
