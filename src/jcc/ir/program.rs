use crate::jcc::asm;

use super::function::Function;

#[derive(Debug)]
pub struct Program {
    entrypoint: Function,
}

impl Program {
    pub fn new(entrypoint: Function) -> Program {
        Program { entrypoint }
    }
}

impl Into<asm::Program> for Program {
    fn into(self) -> asm::Program {
        asm::Program::new(self.entrypoint.into())
    }
}
