#[allow(dead_code)]
#[derive(Debug)]
pub struct UnaryOperator {
    kind: UnaryOperatorKind,
}

impl UnaryOperator {
    pub fn new(kind: UnaryOperatorKind) -> UnaryOperator {
        UnaryOperator { kind }
    }
}

#[derive(Debug)]
pub enum UnaryOperatorKind {
    Complement,
    Negate,
}
