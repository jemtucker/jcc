use crate::jcc::asm;

use super::instruction::Instruction;

#[derive(Debug)]
pub struct Function {
    name: String,
    instructions: Vec<Instruction>,
}

impl Function {
    pub fn new(name: String, instructions: Vec<Instruction>) -> Function {
        Function { name, instructions }
    }
}

impl Into<asm::Function> for Function {
    fn into(self) -> asm::Function {
        let instr = self
            .instructions
            .into_iter()
            .flat_map(|i| Into::<Vec<asm::Instruction>>::into(i))
            .collect();

        asm::Function::new(self.name, instr)
    }
}
