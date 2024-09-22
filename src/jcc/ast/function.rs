use crate::jcc::asm;

use super::Return;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Function {
    name: String,
    body: Return,
}

impl Function {
    pub fn new(name: &str, body: Return) -> Function {
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
