#[allow(dead_code)]
#[derive(Debug)]
pub struct Constant {
    value: i64,
}

impl Constant {
    pub fn new(value: i64) -> Constant {
        Constant { value }
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}
