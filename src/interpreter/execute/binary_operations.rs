use crate::interpreter::errors::ErrType::LogicError;
use crate::interpreter::errors::*;
use crate::interpreter::tokens::*;

pub fn eval_err<A>(text: String, pos: usize) -> LoxResult<A> {
    LoxError::new_err(text.to_string(), pos, LogicError)
}

pub fn handle(op: &Token, acc: LoxValue, val: LoxValue, curr_pos: usize) -> LoxResult<LoxValue> {
    match op.as_punct()? {
        Punct::Star => star(acc, val, curr_pos),
        Punct::Plus => plus(acc, val, curr_pos),
        Punct::Minus => minus(acc, val, curr_pos),
        _ => eval_err(
            format!("Dude, {:?} is not a valid operation!", op),
            curr_pos,
        ),
    }
}

fn plus(acc: LoxValue, val: LoxValue, pos: usize) -> LoxResult<LoxValue> {
    match (&acc, &val) {
        (LoxValue::Integer(x), LoxValue::Integer(y)) => Ok(LoxValue::Integer(x + y)),
        (LoxValue::Boolean(x), LoxValue::Boolean(y)) => Ok(LoxValue::Boolean(*x || *y)),
        _ => eval_err(
            format!(
                "How do you expect me to perform + on {:?} and {:?})",
                acc, val
            ),
            pos,
        ),
    }
}

fn star(acc: LoxValue, val: LoxValue, pos: usize) -> LoxResult<LoxValue> {
    match (&acc, &val) {
        (LoxValue::Integer(x), LoxValue::Integer(y)) => Ok(LoxValue::Integer(x * y)),
        (LoxValue::Boolean(x), LoxValue::Boolean(y)) => Ok(LoxValue::Boolean(*x && *y)),
        _ => eval_err(
            format!(
                "How do you expect me to perform * on {:?} and {:?})",
                acc, val
            ),
            pos,
        ),
    }
}

fn minus(acc: LoxValue, val: LoxValue, pos: usize) -> LoxResult<LoxValue> {
    match (&acc, &val) {
        (LoxValue::Integer(x), LoxValue::Integer(y)) => Ok(LoxValue::Integer(x - y)),
        (LoxValue::Boolean(x), LoxValue::Boolean(y)) => Ok(LoxValue::Boolean(*x && !*y)),
        _ => eval_err(
            format!(
                "How do you expect me to perform - on {:?} and {:?})",
                acc, val
            ),
            pos,
        ),
    }
}
