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
