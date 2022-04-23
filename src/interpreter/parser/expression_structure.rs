use super::super::super::interpreter::parser::Token;



pub struct Expr {}
pub struct Eqlty {}
pub struct Comp {}
pub struct Elem {}

pub trait ParsingElement {}

impl ParsingElement for Expr {}
impl ParsingElement for Eqlty {}
impl ParsingElement for Comp {}
impl ParsingElement for Elem {}

impl Default for Expr  { fn default() -> Self { Expr  {} } }
impl Default for Eqlty { fn default() -> Self { Eqlty {} } }
impl Default for Comp  { fn default() -> Self { Comp  {} } }
impl Default for Elem  { fn default() -> Self { Elem  {} } }

pub enum Node<A> {
    Bi(BiNode<A>),
    Unary(UnaryNode<A>),
    Leaf(Leaf<A>)
}

pub struct BiNode<A> {
    _sub_node_type_holder: Box<A>,
    left: Box<Node<A>>,
    op: Token,
    right: Box<Node<A>>
}

pub struct UnaryNode<A> {
    _sub_node_type_holder: Box<A>,
   op: Token,
   right: Box<Node<A>
}

pub struct Leaf<A> {
    _token_type_holder: Box<A>,
    value: Token
}

impl BiNode<A> where A: ParsingElement {
    pub fn new(left: Node<A>, op: Token, right: Node<A>) -> Self {
        BiNode { left: Box::new(left), op: op, right: Box::new(right), _sub_node_type_holder: Box::new(A::default()) }
    }
    
}
