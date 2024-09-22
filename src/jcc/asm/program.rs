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
