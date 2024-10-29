use std::fmt::Display;

use super::instruction::Instruction;

#[derive(Debug)]
pub struct Function {
    name: String,
    instructions: Vec<Instruction>,
}

impl Function {
    /// Construct a new `Function` with the given name and set of instructions
    pub fn new(name: String, instructions: Vec<Instruction>) -> Function {
        Function { name, instructions }
    }

    /// Prepend the specified `instructions` onto the start of this `Function`
    pub fn prepend(self, instructions: Vec<Instruction>) -> Function {
        Function {
            name: self.name,
            instructions: instructions
                .into_iter()
                .chain(self.instructions.into_iter())
                .collect(),
        }
    }

    pub fn map<F: FnMut(Instruction) -> Instruction>(self, f: F) -> Function {
        Function {
            name: self.name,
            instructions: self.instructions.into_iter().map(f).collect(),
        }
    }

    pub fn flat_map<F: FnMut(Instruction) -> Vec<Instruction>>(self, f: F) -> Function {
        Function {
            name: self.name,
            instructions: self.instructions.into_iter().flat_map(f).collect(),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = "_".to_owned() + &self.name; // osx only

        writeln!(f, "   .globl {}", name)?;
        writeln!(f, "{}:", name)?;
        writeln!(f, "   pushq %rbp")?;
        writeln!(f, "   movq %rsp, %rbp")?;

        for instr in &self.instructions {
            write!(f, "{}", instr)?;
        }

        Ok(())
    }
}
