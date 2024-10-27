use crate::jcc::asm::{Instruction, Operand};

use super::{unary_operator::UnaryOperator, Constant};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Expression {
    kind: ExpressionKind,
}

impl Expression {
    pub fn new(kind: ExpressionKind) -> Expression {
        Expression { kind }
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
    Unary(UnaryOperator, Box<Expression>),
}

impl Into<Vec<Instruction>> for ExpressionKind {
    fn into(self) -> Vec<Instruction> {
        match self {
            Self::Constant(c) => vec![Instruction::Mov {
                src: Operand::Imm(c.value()),
                dst: Operand::Register,
            }],

            Self::Unary(_, _) => unimplemented!(),
        }
    }
}
