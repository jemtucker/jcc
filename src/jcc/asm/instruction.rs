use super::operand::Operand;

/// Instruction is a single x86 instruction
#[derive(Debug)]
pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Ret,
}
