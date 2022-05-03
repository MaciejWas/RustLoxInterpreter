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
        let prelude: String = text[(self.pos - 10)..(self.pos + 10)]
            .to_string();

        [prelude, self.msg.clone()].join("")
    }

    pub fn new_err<A>(msg: String, pos: usize, err_type: ErrType) -> LoxResult<A> {
        Err( Self { msg: msg, err_type: err_type, pos: pos })
    }
}

pub type LoxResult<A> = Result<A, LoxError>;
pub type ErrBuilder = fn(usize) -> LoxError;