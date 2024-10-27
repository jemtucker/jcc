use crate::jcc::asm;

use super::Statement;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Function {
    name: String,
    body: Statement,
}

impl Function {
    pub fn new(name: &str, body: Statement) -> Function {
        Function {
            name: name.to_owned(),
            body,
        }
    }
}

impl Into<asm::Function> for Function {
    fn into(self) -> asm::Function {
        asm::Function::new(self.name, self.body.into())
    }
}
