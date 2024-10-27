use std::fmt::Display;

use super::{operand::Operand, unary_operator::UnaryOperator};

/// Instruction is a single x86 instruction
#[derive(Debug)]
pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Unary(UnaryOperator, Operand),
    AllocateStack(usize),
    Ret,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Mov { src, dst } => {
                write!(f, "movl {}, {}", src, dst)
            }
            Instruction::Unary(op, operand) => {
                write!(f, "{} {}", op, operand)
            }
            Instruction::AllocateStack(n) => {
                unimplemented!()
            }
            Instruction::Ret => {
                write!(f, "ret")
            }
        }
    }
}
