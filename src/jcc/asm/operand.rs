use std::fmt::Display;

/// Operand
#[derive(Debug)]
pub enum Operand {
    Imm(i64),
    Register,
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Imm(i) => write!(f, "${}", i),
            Self::Register => write!(f, "%eax"),
        }
    }
}
