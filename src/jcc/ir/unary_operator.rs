use crate::jcc::asm;

#[derive(Debug)]
pub enum UnaryOperator {
    Complement,
    Negate,
}

impl Into<asm::UnaryOperator> for UnaryOperator {
    fn into(self) -> asm::UnaryOperator {
        match self {
            Self::Complement => asm::UnaryOperator::Not,
            Self::Negate => asm::UnaryOperator::Negate,
        }
    }
}
