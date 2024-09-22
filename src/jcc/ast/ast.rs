use super::Program;

#[derive(Debug)]
pub struct AST {
    program: Program,
}

impl AST {
    /// Construct a new AST
    pub fn new(program: Program) -> AST {
        AST { program }
    }
}
