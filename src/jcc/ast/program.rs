use crate::jcc::ir;

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

impl Into<ir::Program> for Program {
    fn into(self) -> ir::Program {
        ir::Program::new(self.function.into())
    }
}
