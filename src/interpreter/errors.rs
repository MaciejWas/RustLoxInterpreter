use std::cmp::{max, min};

#[derive(Debug)]
pub enum ErrType {
    ParsingErr, EvalErr, TokenizingErr, ScanningErr, LogicError
}

#[derive(Debug)]
pub struct LoxError {
    pub msg: String,
    pub err_type: ErrType,
    pub pos: usize
}

impl LoxError {
    pub fn generate_err_msg(&self, text: &String) -> String {
        let start = max(self.pos as i32 - 10, 0) as usize;
        let end = min(self.pos+10, text.len());
        let prelude: String = text[start..end]
            .to_string();

        [prelude, self.msg.clone()].join("\n")
    }

    pub fn new_err<A>(msg: String, pos: usize, err_type: ErrType) -> LoxResult<A> {
        Err( Self { msg: msg, err_type: err_type, pos: pos })
    }
}

pub type LoxResult<A> = Result<A, LoxError>;
pub type ErrBuilder = fn(usize) -> LoxError;