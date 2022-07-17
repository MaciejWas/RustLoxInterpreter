use super::visitor::*;
use crate::interpreter::parser::Position;
use crate::interpreter::parser::Token;

pub type Program = Vec<Statement>;
pub type SubRules<A> = Vec<(Token, A)>;

#[derive(Debug, Clone)]
pub enum Statement {
    Expr(Expr),
    Print(Expr),
    If(Expr, Program),
    Let(LVal, RVal),
    WhileLoop(Expr, Program),
    Fun(Position, FunctionDefinition),
    Return(Expr),
    Class(ClassDefinition),
}

#[derive(Debug, Clone)]
pub enum StatementKind {
    Expr,
    Print,
    If,
    Let,
    WhileLoop,
    Fun,
    Return,
    Class,
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub name: Token,
    pub args: Vec<Token>,
    pub body: Program,
}

#[derive(Debug, Clone)]
pub struct ClassDefinition {
    pub name: Token,
    pub fields: Vec<(LVal, RVal)>,
    pub methods: Vec<FunctionDefinition>,
}

#[derive(Debug, Clone)]
pub struct LVal {
    pub identifier: Token,
}

#[derive(Debug, Clone)]
pub struct RVal {
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Eqlty(Eqlty),
    Call(Token, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub struct Eqlty {
    pub first: Comp,
    pub rest: SubRules<Comp>,
}

#[derive(Debug, Clone)]
pub struct Comp {
    pub first: Term,
    pub rest: SubRules<Term>,
}

#[derive(Debug, Clone)]
pub struct Term {
    pub first: Factor,
    pub rest: SubRules<Factor>,
}

#[derive(Debug, Clone)]
pub struct Factor {
    pub first: Unary,
    pub rest: SubRules<Unary>,
}

#[derive(Debug, Clone)]
pub enum Unary {
    Final(Option<Token>, Token),
    Recursive(Option<Token>, Box<Expr>),
    Call(Option<Token>, Token, Vec<Box<Expr>>),
}

#[derive(Debug, Clone)]
pub enum UnaryKind {
    Final(bool),
    Recursive(bool),
    Call(bool),
}

// #################################

pub trait FromSubRules<A> {
    fn from_sub(first: A, rest: SubRules<A>) -> Self;
}

impl FromSubRules<Comp> for Eqlty {
    fn from_sub(first: Comp, rest: SubRules<Comp>) -> Self {
        Eqlty {
            first: first,
            rest: rest,
        }
    }
}

impl FromSubRules<Term> for Comp {
    fn from_sub(first: Term, rest: SubRules<Term>) -> Self {
        Comp {
            first: first,
            rest: rest,
        }
    }
}

impl FromSubRules<Factor> for Term {
    fn from_sub(first: Factor, rest: SubRules<Factor>) -> Self {
        Term {
            first: first,
            rest: rest,
        }
    }
}

impl FromSubRules<Unary> for Factor {
    fn from_sub(first: Unary, rest: SubRules<Unary>) -> Self {
        Factor {
            first: first,
            rest: rest,
        }
    }
}

// #################################

impl Acceptor for Statement {
    fn accept<B>(&self, mut v: Box<dyn Visitor<Self, B>>) -> B {
        v.visit(self)
    }
}

impl Acceptor for Expr {
    fn accept<B>(&self, mut v: Box<dyn Visitor<Self, B>>) -> B {
        v.visit(self)
    }
}

impl Acceptor for Eqlty {
    fn accept<B>(&self, mut v: Box<dyn Visitor<Self, B>>) -> B {
        v.visit(self)
    }
}

impl Acceptor for Comp {
    fn accept<B>(&self, mut v: Box<dyn Visitor<Self, B>>) -> B {
        v.visit(self)
    }
}

impl Acceptor for Factor {
    fn accept<B>(&self, mut v: Box<dyn Visitor<Self, B>>) -> B {
        v.visit(self)
    }
}

impl Acceptor for Term {
    fn accept<B>(&self, mut v: Box<dyn Visitor<Self, B>>) -> B {
        v.visit(self)
    }
}

impl Acceptor for Unary {
    fn accept<B>(&self, mut v: Box<dyn Visitor<Self, B>>) -> B {
        v.visit(self)
    }
}

// #################################

pub trait NamedType {
    fn type_name(&self) -> String;
}

impl NamedType for Statement {
    fn type_name(&self) -> String {
        match self {
            Self::Class(_) => "ClassDef",
            Self::Print(_) => "Statement",
            Self::Expr(_) => "Expression",
            Self::If(_, _) => "IfStatement",
            Self::Let(_, _) => "LetStmt",
            Self::WhileLoop(_, _) => "WhileLoop",
            Self::Fun(_, _) => "Fun",
            Self::Return(_) => "Return",
        }
        .to_string()
    }
}
impl NamedType for Expr {
    fn type_name(&self) -> String {
        "Expression".to_string()
    }
}
impl NamedType for Eqlty {
    fn type_name(&self) -> String {
        "Equality".to_string()
    }
}
impl NamedType for Comp {
    fn type_name(&self) -> String {
        "Comparison".to_string()
    }
}
impl NamedType for Term {
    fn type_name(&self) -> String {
        "Term".to_string()
    }
}
impl NamedType for Factor {
    fn type_name(&self) -> String {
        "Factor".to_string()
    }
}
impl NamedType for Unary {
    fn type_name(&self) -> String {
        "Unary".to_string()
    }
}
