use std::fmt::Display;

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

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = "_".to_owned() + &self.name; // osx only

        writeln!(f, "   .globl {}", name)?;
        writeln!(f, "{}:", name)?;

        for instr in &self.instructions {
            writeln!(f, "   {}", instr)?;
        }

        Ok(())
    }
}
