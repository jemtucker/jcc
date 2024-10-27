use crate::jcc::ir;

#[derive(Debug)]
pub struct UnaryOperator {
    kind: UnaryOperatorKind,
}

impl UnaryOperator {
    pub fn new(kind: UnaryOperatorKind) -> UnaryOperator {
        UnaryOperator { kind }
    }

    pub fn as_ir_op(&self) -> ir::UnaryOperator {
        match &self.kind {
            UnaryOperatorKind::Complement => ir::UnaryOperator::Complement,
            UnaryOperatorKind::Negate => ir::UnaryOperator::Negate,
        }
    }
}

#[derive(Debug)]
pub enum UnaryOperatorKind {
    Complement,
    Negate,
}
