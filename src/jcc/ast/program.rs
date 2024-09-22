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
