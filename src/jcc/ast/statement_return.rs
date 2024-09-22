use super::Constant;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Return {
    value: Constant,
}

impl Return {
    pub fn new(value: Constant) -> Return {
        Return { value }
    }
}
