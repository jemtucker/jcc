use crate::jcc::asm::{Instruction, Operand};

use super::Constant;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Expression {
    kind: ExpressionKind,
}

impl Expression {
    pub fn new(constant: Constant) -> Expression {
        Expression {
            kind: ExpressionKind::Constant(constant),
        }
    }
}

impl Into<Vec<Instruction>> for Expression {
    fn into(self) -> Vec<Instruction> {
        self.kind.into()
    }
}

#[derive(Debug)]
pub enum ExpressionKind {
    Constant(Constant),
}

impl Into<Vec<Instruction>> for ExpressionKind {
    fn into(self) -> Vec<Instruction> {
        match self {
            Self::Constant(c) => vec![Instruction::Mov {
                src: Operand::Imm(c.value()),
                dst: Operand::Register,
            }],
        }
    }
}
