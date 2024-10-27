use crate::jcc::asm::Operand;

#[derive(Debug, Clone)]
pub enum Value {
    Constant(i64),
    Var(String),
}

impl Into<Operand> for Value {
    fn into(self) -> Operand {
        match self {
            Self::Constant(i) => Operand::Imm(i),
            Self::Var(s) => Operand::PseudoReg(s),
        }
    }
}
