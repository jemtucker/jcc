use crate::jcc::asm::{self, Operand, Register};

use super::{UnaryOperator, Value};

/// Instruction is a single x86 instruction
#[derive(Debug)]
pub enum Instruction {
    Return(Value),
    Unary(UnaryOperator, Value, Value),
}

impl Into<Vec<asm::Instruction>> for Instruction {
    fn into(self) -> Vec<asm::Instruction> {
        match self {
            Self::Return(v) => {
                vec![
                    asm::Instruction::Mov {
                        src: v.into(),
                        dst: Operand::Reg(Register::AX),
                    },
                    asm::Instruction::Ret,
                ]
            }
            Self::Unary(op, src, dst) => {
                vec![
                    asm::Instruction::Mov {
                        src: src.into(),
                        dst: dst.clone().into(),
                    },
                    asm::Instruction::Unary(op.into(), dst.into()),
                ]
            }
        }
    }
}
