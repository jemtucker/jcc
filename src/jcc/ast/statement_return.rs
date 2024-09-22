use crate::jcc::asm::{Instruction, Operand};

use super::Constant;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Return {
    value: Constant,
}

impl Return {
    pub fn new(value: Constant) -> Return {
        Return { value }
    }
}

impl Into<Vec<Instruction>> for Return {
    fn into(self) -> Vec<Instruction> {
        vec![
            Instruction::Mov {
                src: Operand::Imm(self.value.value()),
                dst: Operand::Register,
            },
            Instruction::Ret,
        ]
    }
}
