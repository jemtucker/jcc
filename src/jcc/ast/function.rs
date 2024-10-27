use crate::jcc::ir;

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

impl Into<ir::Function> for Function {
    fn into(self) -> ir::Function {
        ir::Function::new(self.name, self.body.into())
    }
}
