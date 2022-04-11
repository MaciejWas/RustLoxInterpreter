use super::super::super::interpreter::parser::Token;

pub enum FinalOrRecursive<A> {
    Final(A), Recursive(Box<Node<A>>)
}

pub fn final_<A>(a: A) -> FinalOrRecursive<A> {
    FinalOrRecursive::Final(a)
}

pub fn recursive<A>(a: Node<A>) -> FinalOrRecursive<A> {
    FinalOrRecursive::Recursive(Box::new(a))
}

//////////////// ABSTRACT NODE ////////////////
  
pub struct Node<SubNode> {
    left: SubNode,
    op: Token,
    right: FinalOrRecursive<SubNode>
}

impl <SubNode> Node<SubNode> {
    pub fn new(left: SubNode, op: Token, right: FinalOrRecursive<SubNode>) -> Self {
        Node {left: left, op: op, right: right}
    }

    pub fn replace_right(&mut self, op: Token, new_right: SubNode) {
        let curr_right: FinalOrRecursive<SubNode> = self.right;
        match curr_right {
            FinalOrRecursive::Recursive(node) => node.replace_right(op, new_right),
            FinalOrRecursive::Final(node) => curr_right = recursive( Node {left: node, op: op, right: final_(new_right)} )
        };
    }
}

// pub struct NodeBuilder<SubNode> {
//     left: Option<SubNode>,
//     op: Option<Token>,
//     right: Option<FinalOrRecursive<SubNode>>
// }

// impl <SubNode> NodeBuilder<SubNode> {
//     pub fn set_left(&mut self, left: SubNode) {
//         self.left = Some(left)
//     }

//     pub fn update_right(&mut self, right: SubNode) {
//         self
//         }
//     }
// }

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
