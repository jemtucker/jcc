use crate::jcc::asm;

use super::Function;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Program {
    function: Function,
}

impl Program {
    pub fn new(function: Function) -> Program {
        Program { function }
    }
}

impl Into<asm::Program> for Program {
    fn into(self) -> asm::Program {
        asm::Program::new(self.function.into())
    }
}
