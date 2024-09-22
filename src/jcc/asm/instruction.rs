use std::fmt::Display;

use super::operand::Operand;

/// Instruction is a single x86 instruction
#[derive(Debug)]
pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Ret,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Mov { src, dst } => {
                write!(f, "movl {}, {}", src, dst)
            }
            Instruction::Ret => {
                write!(f, "ret")
            }
        }
    }
}
