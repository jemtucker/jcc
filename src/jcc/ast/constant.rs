#[allow(dead_code)]
#[derive(Debug)]
pub struct Constant {
    value: u64,
}

impl Constant {
    pub fn new(value: u64) -> Constant {
        Constant { value }
    }
}
