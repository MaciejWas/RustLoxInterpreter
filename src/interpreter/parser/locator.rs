use crate::interpreter::parser::*;
use crate::interpreter::Visitor;


/// Maps an expression to the position of its first token. Slow and temporary solution.
pub struct Locator;

pub fn locate(expr: &Expr) -> Position {
    let mut l = Locator;
    l.visit(expr)
}

impl Visitor<Expr, Position> for Locator {
    fn visit(&mut self, expr: &Expr) -> Position {
        match expr {
            Expr::Eqlty(eqlty) => self.visit(eqlty),
            Expr::Call(_, _) => panic!("TODO: impl"),
        }
    }
}

impl Visitor<Eqlty, Position> for Locator {
    fn visit(&mut self, eqlty: &Eqlty) -> Position {
        self.visit(&eqlty.first)
    }
}

impl Visitor<Comp, Position> for Locator {
    fn visit(&mut self, comp: &Comp) -> Position {
        self.visit(&comp.first)
    }
}

impl Visitor<Term, Position> for Locator {
    fn visit(&mut self, term: &Term) -> Position {
        self.visit(&term.first)
    }
}

impl Visitor<Factor, Position> for Locator {
    fn visit(&mut self, fac: &Factor) -> Position {
        self.visit(&fac.first)
    }
}

impl Visitor<Unary, Position> for Locator {
    fn visit(&mut self, unary: &Unary) -> Position {
        match unary {
            Unary::Final(None, token) => token.pos,
            Unary::Final(Some(op), _) => op.pos,
            Unary::Recursive(None, expr) => self.visit(expr.as_ref()),
            Unary::Recursive(Some(op), expr) => op.pos,
            Unary::Call(Some(op), _, _) => op.pos,
            Unary::Call(None, t, _) => t.pos
        }
    }
}
