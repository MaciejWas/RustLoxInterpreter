use super::super::super::interpreter::parser::Token;

pub struct Tree {
    nodes: Vec<Node>
}

impl Tree {
    fn add(&mut self, node: Node) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    fn get(&self, id: usize) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn add(&mut self, node: Node) {
        let id: usize = self.add(node);
        let mut curr_pos: usize = 0;
        loop {
            let curr_node = self.get(curr_pos);
            if let Some(node) = curr_node {} else {
                
            }
        }
    }
}

pub struct Node {
    left: usize,
    op: Token,
    right: usize
}

pub trait Constructable<A, B> {
    fn new(a: A, op: Token, b: B) -> Node<A>;
}


//////////////// NODES //////////////////////


pub struct ExprNode {
    eq: EqNode
}

pub type EqNode = Node<CompNode>;
 
pub type CompNode = Node<TermNode>;
 
pub type TermNode = Node<FactorNode>;
 
pub type FactorNode = Node<UnaryNode>;

pub struct UnaryNode {
    op: Option<Token>,
    primary: Primary
} impl UnaryNode {
    pub fn of(op: Option<Token>, p: Primary) -> Self { UnaryNode {op: op, primary: p} }
} 


pub struct Primary {
    token: Token
}

    impl Primary {
        pub fn of(t: Token) -> Self { Primary {token: t} }
    }
