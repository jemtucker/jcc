use std::fmt::Display;

use super::Register;

/// Operand
#[derive(Debug)]
pub enum Operand {
    Imm(i64),
    Reg(Register),
    PseudoReg(String),
    Stack(usize),
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Imm(i) => write!(f, "${}", i),
            Self::Reg(r) => write!(f, "{}", r),
            Self::PseudoReg(s) => write!(f, "PSEUDO(%{})", s),
            Self::Stack(r) => unimplemented!(),
        }
    }
}
