use crate::interpreter::parser::Token;

pub struct Single<A> {
    value: A
}

pub struct Many<A> {
    first: A,
    rest: Vec<(Token, A)>
}

pub struct Unary<A> {
   op: Token,
   right: A
}

pub struct Final {
    value: Token
}

pub type ExprRule    = Single<EqltyRule>;
pub type EqltyRule   = Many<CompRule>;
pub type CompRule    = Many<TermRule>;
pub type TermRule    = Many<FactorRule>;
pub type FactorRule  = Many<UnaryRule>;
pub type UnaryRule   = Unary<PrimaryRule>;
pub type PrimaryRule = Final;
