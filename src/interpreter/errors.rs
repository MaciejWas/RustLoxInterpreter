//! Provides `LoxError` class used for scanning/parsing/runtime errors as well as a builder utility class.

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
    /// Generates a 3-line error message shown directly in the CLI. Looks smth like this:
    /// |
    /// | [line number] code
    /// |                 ^ error message
    ///
    pub fn generate_err_msg(&self, program: &String) -> String {
        let line = program.lines().nth(self.pos.line).unwrap_or_else(|| {
            panic!(
                "Failed to generate error message: line {} is out of range.",
                self.pos.line
            )
        });

        let top_line = " |\n".to_string();
        let middle_line_base = format!(" | [{}] ", self.pos.line + 1);
        let bottom_line_pointer = " "
            .to_string()
            .repeat(self.pos.line_pos + middle_line_base.len() - " | ".len())
            + "^";
        let middle_line = middle_line_base + line + "\n";
        let bottom_line = " | ".to_string() + &bottom_line_pointer + " " + &self.msg + "\n";

        return [top_line, middle_line, bottom_line].join("");
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
        self.pos = Some(pos.clone());
        self
    }

    pub fn without_pos(mut self) -> Self {
        self.pos = Some(Position {
            line: 0,
            line_pos: 0,
        }); // TODO: make it suck less
        self
    }

    pub fn at(mut self, pos: Position) -> Self {
        self.pos = Some(pos);
        self
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

    pub fn cant_perform_a_on_b_and_c<A: std::fmt::Debug, B: std::fmt::Debug, C: std::fmt::Debug>(
        self,
        a: A,
        b: B,
        c: C,
    ) -> ErrBuilder {
        self.with_message(format!("Can't perform {:?} on {:?} and {:?}", a, b, c))
    }

    pub fn reset(mut self) -> Self {
        self.err_type = None;
        self.message = None;
        self.while_info = None;
        self.pos = None;
        self
    }

    pub fn to_result<A>(self) -> LoxResult<A> {
        Err(self.build())
    }

    pub fn build(self) -> LoxError {
        let self_repr = format!("{:?}", &self);
        let msg_core = &self.message.unwrap_or_else(|| {
            panic!(
                "ErrBuilder failed, message was not supplied: {:?}",
                self_repr
            )
        });

        let while_info = &self.while_info.map_or("".to_string(), |info| {
            "\nError occured while: ".to_string() + &info
        });

        LoxError {
            msg: msg_core.clone() + &while_info,
            err_type: self.err_type.unwrap_or_else(|| {
                panic!(
                    "ErrBuilder failed: err_type was not supplied: {:?}",
                    self_repr
                )
            }),
            pos: self.pos.unwrap_or_else(|| {
                panic!("ErrBuilder failed: pos was not supplied: {:?}", self_repr)
            }),
        }
    }
}
