use crate::interpreter::errors::ErrType::LogicError;
use crate::interpreter::errors::{LoxError, LoxResult};
use crate::interpreter::execute::binary_operations;
use crate::interpreter::execute::binary_operations::eval_err;
use crate::interpreter::parser::structure::*;
use crate::interpreter::parser::visitor::*;
use crate::interpreter::tokens::{LoxValue, Punct, Token};
use std::collections::HashMap;

pub struct Executor {
    pub state: HashMap<String, LoxValue>,
}

impl Visitor<Program, LoxResult<()>> for Executor {
    fn visit(&mut self, p: &Program) -> LoxResult<()> {
        for stmt in p.iter() {
            self.visit(stmt)?;
        }
        Ok(())
    }
}

impl Visitor<Statement, LoxResult<()>> for Executor {
    fn visit(&mut self, stmt: &Statement) -> LoxResult<()> {
        match stmt {
            Statement::ExprStmt(_) => {
                println!("I am laze heheheheh")
            }
            Statement::PrintStmt(expr) => {
                let evaluated = self.visit(expr)?;
                println!("From print statement: `{:?}`", evaluated);
            }
            Statement::IfStmt(cond, program) => {
                if LoxValue::Boolean(true) == self.visit(cond)? {
                    self.visit(program);
                }
            }
        }
        Ok(())
    }
}

impl Visitor<Expr, LoxResult<LoxValue>> for Executor {
    fn visit(&mut self, expr: &Expr) -> LoxResult<LoxValue> {
        match expr {
            Expr::Eqlty(eqlty) => self.visit(eqlty),
        }
    }
}

impl Visitor<Eqlty, LoxResult<LoxValue>> for Executor {
    fn visit(&mut self, eqlty: &Eqlty) -> LoxResult<LoxValue> {
        let first_evaluated: LoxResult<LoxValue> = self.visit(&eqlty.first);
        eqlty
            .rest
            .iter()
            .map(|(op, a)| (op, self.visit(a)))
            .fold(first_evaluated, eval_fold)
    }
}

impl Visitor<Comp, LoxResult<LoxValue>> for Executor {
    fn visit(&mut self, comp: &Comp) -> LoxResult<LoxValue> {
        let first_evaluated = self.visit(&comp.first);
        comp.rest
            .iter()
            .map(|(op, a)| (op, self.visit(a)))
            .fold(first_evaluated, eval_fold)
    }
}

impl Visitor<Term, LoxResult<LoxValue>> for Executor {
    fn visit(&mut self, term: &Term) -> LoxResult<LoxValue> {
        let first_evaluated = self.visit(&term.first);
        term.rest
            .iter()
            .map(|(op, a)| (op, self.visit(a)))
            .fold(first_evaluated, eval_fold)
    }
}

impl Visitor<Factor, LoxResult<LoxValue>> for Executor {
    fn visit(&mut self, fac: &Factor) -> LoxResult<LoxValue> {
        let first_evaluated = self.visit(&fac.first);
        fac.rest
            .iter()
            .map(|(op, a)| (op, self.visit(a)))
            .fold(first_evaluated, eval_fold)
    }
}

impl Visitor<Unary, LoxResult<LoxValue>> for Executor {
    fn visit(&mut self, unary: &Unary) -> LoxResult<LoxValue> {
        match unary {
            Unary::Final(None, token) => get_value(token, &self.state),
            Unary::Final(Some(op), token) => {
                unary_op(op.as_punct()?, get_value(token, &self.state)?, op.pos())
            }

            Unary::Recursive(None, expr) => self.visit(expr.as_ref()),
            Unary::Recursive(Some(op), expr) => {
                unary_op(op.as_punct()?, self.visit(expr.as_ref())?, op.pos())
            }
        }
    }
}

fn get_value(token: &Token, state: &HashMap<String, LoxValue>) -> LoxResult<LoxValue> {
    match token {
        Token::IdentifierToken(identifier, pos) => {
            return Ok(state.get(identifier).unwrap().clone())
        }
        Token::ValueToken(lox_val, pos) => return Ok(lox_val.clone()),
        Token::KwdToken(kwd, pos) => {
            LoxError::new_err(format!("wtf {:?} is not unary", kwd), *pos, LogicError)
        }
        Token::PunctToken(punct, pos) => {
            LoxError::new_err(format!("wtf {:?} is not unary", punct), *pos, LogicError)
        }
    }
}

fn eval_fold(acc: LoxResult<LoxValue>, next: (&Token, LoxResult<LoxValue>)) -> LoxResult<LoxValue> {
    let acc: LoxValue = acc?;
    let curr_pos = next.0.pos();

    let (op, val) = next;
    let val: LoxValue = val?;

    return binary_operations::handle(op, acc, val, curr_pos);
}

fn unary_op(op: Punct, right: LoxValue, pos: usize) -> LoxResult<LoxValue> {
    match op {
        Punct::Minus => match right {
            LoxValue::Integer(x) => Ok(LoxValue::from(-x)),
            _ => eval_err(
                format!(
                    "applying {:?} on {:?} as an unary operator is not supported.",
                    op, right
                ),
                pos,
            ),
        },
        _ => eval_err(format!("{:?} is not a valid unary operator.", op), pos),
    }
}
