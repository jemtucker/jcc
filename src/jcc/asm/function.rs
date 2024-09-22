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
