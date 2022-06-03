use std::cmp::{max, min};

pub type LoxResult<A> = Result<A, LoxError>;

#[derive(Clone, Debug)]
pub enum ErrType {
    ParsingErr,
    EvalErr,
    TokenizingErr,
    ScanningErr,
    LogicError,
    InterpreterError,
}

fn find_line_with_pos(text: &String, pos: usize) -> (String, usize) {

    let mut cum = 0;
    for line in text.splitn(100000, '\n') {
        cum += line.len();
        println!("{}", line);
        println!("{} vs {} / {}", pos, cum, text.len());
        if cum > pos {
            return (line.to_string(), pos - cum)
        }
    };

    panic!("Line with pos not found.")
}

#[derive(Clone, Debug)]
pub struct LoxError {
    pub msg: String,
    pub err_type: ErrType,
    pub pos: usize,
}

impl LoxError {
    pub fn generate_err_msg(&self, program: &String) -> String {
        let (line, line_pos) = find_line_with_pos(program, self.pos);
        let pointer: String = "-".to_string().repeat(line_pos) + "^";
        return [line, pointer, self.msg.clone()].join("\n")
    }

    pub fn new_err<A>(msg: String, pos: usize, err_type: ErrType) -> LoxResult<A> {
        Err(Self {
            msg: msg,
            err_type: err_type,
            pos: pos,
        })
    }
}

#[derive(Clone, Debug)]
pub struct ErrBuilder {
    err_type: Option<ErrType>,
    message: Option<String>,
    while_info: Option<String>,
    pos: Option<usize>,
}

impl ErrBuilder {
    pub fn new() -> Self {
        ErrBuilder {
            err_type: None,
            while_info: None,
            message: None,
            pos: None,
        }
    }

    pub fn with_type(mut self, err_type: ErrType) -> Self {
        self.err_type = Some(err_type);
        self
    }

    pub fn with_message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    pub fn with_pos(mut self, pos: usize) -> Self {
        self.pos = Some(pos);
        self
    }

    pub fn at(pos: usize) -> Self {
        ErrBuilder {
            err_type: None,
            message: None,
            while_info: None,
            pos: Some(pos),
        }
    }

    pub fn expected_but_found<E, F>(mut self, expected: E, found: F) -> Self
    where
        E: std::fmt::Debug,
        F: std::fmt::Debug,
    {
        self.message = Some(format!("Expected {:?}, but found {:?}.", expected, found));
        self
    }

    pub fn is_not<A, B>(mut self, a: A, b: B) -> Self
    where
        A: std::fmt::Debug,
        B: std::fmt::Debug,
    {
        self.message = Some(format!("{:?} is not {:?}.", a, b));
        self
    }

    pub fn while_<A>(mut self, msg: A) -> Self
    where
        A: std::fmt::Debug,
    {
        self.while_info = Some(format!("{:?}", msg));
        self
    }

    pub fn expected_found_nothing<E>(mut self, expected: E) -> Self
    where
        E: std::fmt::Debug,
    {
        self.message = Some(format!("Expected {:?}, but found nothing.", expected));
        self
    }

    pub fn build(self) -> LoxError {
        let msg_core = self
            .message
            .unwrap_or_else(|| panic!("ErrBuilder failed: message was not supplied"));

        let while_info = self
            .while_info
            .map_or("".to_string(), |info| "\n\t While: ".to_string() + &info);

        LoxError {
            msg: msg_core + &while_info,
            err_type: self
                .err_type
                .unwrap_or_else(|| panic!("ErrBuilder failed: err_type was not supplied")),
            pos: self
                .pos
                .unwrap_or_else(|| panic!("ErrBuilder failed: pos was not supplied")),
        }
    }
}

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
