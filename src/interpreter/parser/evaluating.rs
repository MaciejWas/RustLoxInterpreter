use super::expression_structure::*;
use crate::interpreter::tokens::{
    token_types::{
        LoxValue,
        lox_int,
        lox_bool,
        Punct,
    },
    Token,
    Token::{
        ValueToken, 
        PunctToken
    }
};
use crate::interpreter::errors::{eval_err, LoxResult};


fn apply(op: Punct, right: LoxValue) -> LoxResult<LoxValue> {
    match op {
        Punct::Minus => match right {
            LoxValue::Integer(x) => Ok(lox_int(-x)),
            _ => eval_err(format!("applying {:?} on {:?} as an unary operator is not supported", op, right))
        },
        _ => eval_err(format!("{:?} is not a valid unary operator", op))
    }
}

fn eval_fold(acc: LoxResult<LoxValue>, next: (&Token, LoxResult<LoxValue>)) -> LoxResult<LoxValue> {
    let acc: LoxValue = acc?;

    let (op, val) = next;
    let op: Punct = op.as_punct()?;
    let val: LoxValue = val?;

    let result = match op {
        Punct::Star => match (acc, val) {
            (LoxValue::Integer(x), LoxValue::Integer(y)) => Ok(LoxValue::Integer(x * y)),
            (LoxValue::Boolean(x), LoxValue::Boolean(y)) => Ok(LoxValue::Boolean(x && y)),
            _ => eval_err("Shit".to_string())
        },
        Punct::Plus => match (acc, val) {
            (LoxValue::Integer(x), LoxValue::Integer(y)) => Ok(LoxValue::Integer(x + y)),
            (LoxValue::Boolean(x), LoxValue::Boolean(y)) => Ok(LoxValue::Boolean(x || y)),
            _ => eval_err("Shit".to_string())

        },
        _ => eval_err("Shit".to_string())
    };

    return result;
}

pub trait Evaluate {
    fn eval(&self) -> LoxResult<LoxValue>;
}

impl <A> Evaluate for Single<A> where A: Evaluate {
    fn eval(&self) -> LoxResult<LoxValue> {
        self.value.eval()
    }
}

impl <A> Evaluate for Many<A> where A: Evaluate + std::fmt::Debug {
    fn eval(&self) -> LoxResult<LoxValue> {

        let first_evaluated = self.first.eval();

        self.rest
            .iter()
            .map(|(op, a)| (op, a.eval()))
            .fold(
                first_evaluated,
                eval_fold
            )
    }
}

impl Evaluate for Unary {
    fn eval(&self) -> LoxResult<LoxValue> {
        let val = self.right.as_lox_value()?;

        if let Some(op) = &self.op {
            let op = op.as_punct()?;
            return apply(op, val)
        };

        Ok(val)

    }
}
