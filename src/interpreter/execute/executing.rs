//! A Visitor-style executor for `Vec<Statement>`.

use crate::interpreter::execute::executing_function::FuncExecutor;
use crate::interpreter::parser::locator::locate;
use crate::interpreter::tokens::LoxValue;
use crate::interpreter::{
    errors::position::Position,
    errors::LoxResult,
    execute::{
        definitions::{LoxObj, RawLoxObject},
        operations::{binary_operations, eval_err, unary_op},
    },
    parser::structure::*,
    parser::visitor::*,
    tokens::position_of,
    tokens::Token,
};
use std::iter::zip;

use super::state::State;

pub struct Executor {
    pub state: State,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            state: State::new(),
        }
    }

    pub fn from(state: State) -> Self {
        Executor { state }
    }

    /// Creates new scope, does F, pops last scope. Generally used every time the executor goes into curly brackets.
    pub fn scoped<F, A>(&mut self, f: F) -> A
    where
        F: Fn(&mut Self) -> A,
    {
        self.state.push_new_scope();
        let result = f(self);
        self.state
            .pop_last_scope()
            .unwrap_or_else(|| panic!("Could not pop last scope!"));
        result
    }
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
                println!("I am lazy heheheheh")
            }
            Statement::PrintStmt(expr) => {
                let evaluated = self.visit(expr)?;
                println!("{}", evaluated.to_string())
            }
            Statement::IfStmt(cond, program) => {
                let cond_evaluated = self.visit(cond)?;
                match Option::<bool>::from(cond_evaluated) {
                    Some(true) => self.scoped(|v| v.visit(program))?,
                    Some(false) => (),
                    None => {
                        return eval_err()
                            .at(locate(&cond))
                            .with_message("could not evaluate if stmt condition".to_string())
                            .to_result()
                    }
                }
            }
            Statement::WhileLoop(cond, program) => loop {
                let cond_evaluated = self.visit(cond)?;
                match Option::<bool>::from(cond_evaluated) {
                    Some(false) => break,
                    None => {
                        return eval_err()
                            .without_pos()
                            .with_message("could not evaluate while stmt condition".to_string())
                            .to_result()
                    }
                    _ => {}
                };

                self.scoped(|v| v.visit(program))?;
            },
            Statement::LetStmt(lval, rval) => {
                let pos = locate(&rval.expr);
                let right_evaluated = self.visit(&rval.expr)?;
                self.state
                    .bind(lval.identifier.clone(), right_evaluated, pos)?;
            }
            Statement::DefStmt(pos, function_definition) => {
                self.state.bind(
                    function_definition.name.clone(),
                    LoxObj::from(function_definition.clone()),
                    *pos,
                )?;
            }
            Statement::Return(expr) => {
                return eval_err()
                    .at(locate(&expr))
                    .with_message("return stmt outside function body".to_string())
                    .to_result()
            }
        }
        Ok(())
    }
}

impl Visitor<Expr, LoxResult<LoxObj>> for Executor {
    fn visit(&mut self, expr: &Expr) -> LoxResult<LoxObj> {
        match expr {
            Expr::Eqlty(eqlty) => self.visit(eqlty),
            Expr::Call(Token::IdentifierToken(function_name, pos), args) => {
                let called_object = self.state.get(function_name, *pos)?;
                let function = called_object.transform(|raw| match raw {
                    RawLoxObject::Fun(function_def) => Ok(function_def),
                    _ => eval_err()
                        .at(*pos)
                        .is_not(raw.to_string(), "callable")
                        .to_result(),
                })?;

                let args_evaluated: LoxResult<Vec<LoxObj>> = args
                    .into_iter()
                    .map(|arg_expr| self.visit(arg_expr))
                    .collect();

                self.state.push_new_scope();

                for (arg_name, arg_evaluated) in zip(function.args, args_evaluated?) {
                    self.state.bind(arg_name, arg_evaluated, *pos);
                }

                FuncExecutor::from(self.clone());

                // match function.ret {
                // Some(expr) => self.visit(&expr),
                Ok(LoxObj::from(LoxValue::from(0)))
                // }
            }
            _ => panic!("todo: impl"),
        }
    }
}

impl Visitor<Eqlty, LoxResult<LoxObj>> for Executor {
    fn visit(&mut self, eqlty: &Eqlty) -> LoxResult<LoxObj> {
        let first_evaluated: LoxResult<LoxObj> = self.visit(&eqlty.first);
        eqlty
            .rest
            .iter()
            .map(|(op, a)| (op, self.visit(a)))
            .fold(first_evaluated, eval_fold)
    }
}

impl Visitor<Comp, LoxResult<LoxObj>> for Executor {
    fn visit(&mut self, comp: &Comp) -> LoxResult<LoxObj> {
        let first_evaluated = self.visit(&comp.first);
        comp.rest
            .iter()
            .map(|(op, a)| (op, self.visit(a)))
            .fold(first_evaluated, eval_fold)
    }
}

impl Visitor<Term, LoxResult<LoxObj>> for Executor {
    fn visit(&mut self, term: &Term) -> LoxResult<LoxObj> {
        let first_evaluated = self.visit(&term.first);
        term.rest
            .iter()
            .map(|(op, a)| (op, self.visit(a)))
            .fold(first_evaluated, eval_fold)
    }
}

impl Visitor<Factor, LoxResult<LoxObj>> for Executor {
    fn visit(&mut self, fac: &Factor) -> LoxResult<LoxObj> {
        let first_evaluated = self.visit(&fac.first);
        fac.rest
            .iter()
            .map(|(op, a)| (op, self.visit(a)))
            .fold(first_evaluated, eval_fold)
    }
}

impl Visitor<Unary, LoxResult<LoxObj>> for Executor {
    fn visit(&mut self, unary: &Unary) -> LoxResult<LoxObj> {
        match unary {
            Unary::Final(None, token) => self.as_lox_obj(token),
            Unary::Final(Some(op), token) => {
                let lox_obj = self.as_lox_obj(token)?;
                lox_obj.apply(|raw| unary_op(op, raw))
            }
            Unary::Recursive(None, expr) => self.visit(expr.as_ref()),
            Unary::Recursive(Some(op), expr) => {
                let evaluated_expression = self.visit(expr.as_ref())?;
                evaluated_expression.apply(|raw| unary_op(op, raw))
            }
        }
    }
}

fn eval_fold(acc: LoxResult<LoxObj>, next: (&Token, LoxResult<LoxObj>)) -> LoxResult<LoxObj> {
    let acc: LoxObj = acc?;
    let (op, val) = next;
    let curr_pos: Position = position_of(op);
    let val: LoxObj = val?;

    if let Some(acc_val) = acc.to_value() {
        if let Some(val_val) = val.to_value() {
            let result = binary_operations::handle(op, acc_val, val_val, curr_pos)?;
            return Ok(LoxObj::from(result));
        }
    }

    panic!("TODO: not panic")
}

impl Executor {
    /// Evaluates token to `LoxObj` if token is an identifier or value
    fn as_lox_obj(&self, token: &Token) -> LoxResult<LoxObj> {
        match token {
            Token::IdentifierToken(id, pos) => self.state.get(id, pos.clone()),
            Token::ValueToken(lox_val, _) => Ok(LoxObj::from(lox_val.clone())),
            _ => Err(eval_err()
                .at(position_of(token))
                .is_not(token, "a lox object")
                .build()),
        }
    }
}
