use crate::jcc::{asm, CodeGenerator};

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

        // TODO tidy all this mess up to move all ir/asm stuff into codegen
        let mut gen = CodeGenerator::new();
        let instructions = gen.codegen(instr);

        asm::Function::new(self.name, instructions)
    }
}
