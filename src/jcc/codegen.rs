use super::{asm::Program, Error};

/// CodeGenerator generates assembly code from an AST
pub struct CodeGenerator {
    program: Program,
}

impl CodeGenerator {
    /// Construct a new CodeGenerator for the provided AST
    pub fn new(program: Program) -> CodeGenerator {
        CodeGenerator { program }
    }

    /// Generate code for the given AST
    pub fn generate(self) -> Result<Program, Error> {
        Ok(self.program.into())
    }
}
