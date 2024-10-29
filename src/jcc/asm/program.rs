use std::fmt::Display;

use super::function::Function;

#[derive(Debug)]
pub struct Program {
    entrypoint: Function,
}

impl Program {
    pub fn new(entrypoint: Function) -> Program {
        Program { entrypoint }
    }

    pub fn map<F: FnMut(Function) -> Function>(self, mut f: F) -> Program {
        Program {
            entrypoint: f(self.entrypoint),
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.entrypoint.fmt(f)
        // TODO if linux add:
        //      .section .note.GNU-stack,"",@progbits
    }
}
