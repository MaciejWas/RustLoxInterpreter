use crate::interpreter::errors::position::Position;

pub mod position;

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

#[derive(Clone, Debug)]
pub struct LoxError {
    pub msg: String,
    pub err_type: ErrType,
    pub pos: Position,
}

impl LoxError {
    pub fn generate_err_msg(&self, program: &String) -> String {
        let line = program.lines().nth(self.pos.line).unwrap_or_else(|| {
            panic!(
                "Failed to generate error message: line {} is out of range.",
                self.pos.line
            )
        });
        let pointer: String = "-".to_string().repeat(self.pos.line_pos) + "^";
        return [line, &pointer, &self.msg].join("\n");
    }
}

#[derive(Clone, Debug)]
pub struct ErrBuilder {
    err_type: Option<ErrType>,
    message: Option<String>,
    while_info: Option<String>,
    pos: Option<Position>,
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

    pub fn of_type(mut self, err_type: ErrType) -> Self {
        self.err_type = Some(err_type);
        self
    }

    pub fn with_message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    pub fn with_pos(mut self, pos: Position) -> Self {
        self.pos = Some(pos);
        self
    }

    pub fn at(pos: Position) -> Self {
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
