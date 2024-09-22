use super::{Constant, Function, Program, Return};

#[allow(dead_code)]
#[derive(Debug)]
pub struct AST {
    root: Node,
}

impl AST {
    /// Construct a new AST guaranteeing that the top level Node is a Program
    pub fn new(program: Program) -> AST {
        AST {
            root: Node::Program(program),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Node {
    Program(Program),
    Function(Function),
    Return(Return),
    Constant(Constant),
}
