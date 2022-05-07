use crate::interpreter::errors::ErrType::LogicError;
use super::expression_structure::*;
use crate::interpreter::tokens::{
    LoxValue,
    Punct,
    Token,
};
use crate::interpreter::errors::{LoxError, LoxResult};

fn eval_err<A>(text: String, pos: usize) -> LoxResult<A> {
    LoxError::new_err(text.to_string(), pos, LogicError)
}

fn apply(op: Punct, right: LoxValue, pos: usize) -> LoxResult<LoxValue> {
    match op {
        Punct::Minus => match right {
            LoxValue::Integer(x) => Ok(LoxValue::from(-x)),
            _ => eval_err(format!("applying {:?} on {:?} as an unary operator is not supported.", op, right), pos)
        },
        _ => eval_err(format!("{:?} is not a valid unary operator.", op), pos)
    }
}

fn eval_fold(acc: LoxResult<LoxValue>, next: (&Token, LoxResult<LoxValue>)) -> LoxResult<LoxValue> {
    let acc: LoxValue = acc?;    
    let curr_pos = next.0.pos();

    let (op, val) = next;
    let val: LoxValue = val?;

    return binary_operations::handle(op, acc, val, curr_pos);
}

mod binary_operations {
    use super::Token;
    use super::Punct;
    use super::LoxValue;
    use super::LoxResult;
    use super::eval_err;

    pub fn handle(op: &Token, acc: LoxValue, val: LoxValue, curr_pos: usize) -> LoxResult<LoxValue> {
        match op.as_punct()? {
            Punct::Star  => star(acc, val, curr_pos),
            Punct::Plus  => plus(acc, val, curr_pos),
            Punct::Minus => minus(acc, val, curr_pos),
            _ => eval_err(format!("Dude, {:?} is not a valid operation!", op), curr_pos)
        }
    }

    fn plus(acc: LoxValue, val: LoxValue, pos: usize) -> LoxResult<LoxValue> {
        match (&acc, &val) {
            (LoxValue::Integer(x), LoxValue::Integer(y)) => Ok(LoxValue::Integer(x + y)),
            (LoxValue::Boolean(x), LoxValue::Boolean(y)) => Ok(LoxValue::Boolean(*x || *y)),
            _ => eval_err(format!("How do you expect me to perform + on {:?} and {:?})", acc, val), pos)
        }
    }
    
    fn star(acc: LoxValue, val: LoxValue, pos: usize) -> LoxResult<LoxValue> {
        match (&acc, &val) {
            (LoxValue::Integer(x), LoxValue::Integer(y)) => Ok(LoxValue::Integer(x * y)),
            (LoxValue::Boolean(x), LoxValue::Boolean(y)) => Ok(LoxValue::Boolean(*x && *y)),
            _ => eval_err(format!("How do you expect me to perform * on {:?} and {:?})", acc, val), pos)
        }
    }

    fn minus(acc:LoxValue, val: LoxValue, pos: usize) -> LoxResult<LoxValue> {
        match (&acc, &val) {
            (LoxValue::Integer(x), LoxValue::Integer(y)) => Ok(LoxValue::Integer(x - y)),
            (LoxValue::Boolean(x), LoxValue::Boolean(y)) => Ok(LoxValue::Boolean(*x && !y)),
            _ => eval_err(format!("How do you expect me to perform - on {:?} and {:?})", acc, val), pos)
        }
    }
}

pub trait Evaluate {
    fn eval(&self) -> LoxResult<LoxValue>;
}

impl <A> Evaluate for Single<A> where A: Evaluate {
    fn eval(&self) -> LoxResult<LoxValue> {
        self.value.eval()
    } // I love ya!
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

        if let Some(tok) = &self.op {
            return apply(tok.as_punct()?, val, self.op.clone().unwrap().pos())
        };

        Ok(val)

    }
}
