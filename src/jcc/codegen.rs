use std::collections::HashMap;

use crate::jcc::asm::Register;

use super::asm::{Instruction, Operand};

pub struct CodeGenerator {
    stack_offsets: HashMap<String, Operand>,
}

impl CodeGenerator {
    pub fn new() -> CodeGenerator {
        CodeGenerator {
            stack_offsets: HashMap::new(),
        }
    }

    pub fn codegen(&mut self, instrs: Vec<Instruction>) -> Vec<Instruction> {
        let mut out = instrs
            .into_iter()
            // First pass - convert pseudo registers to stack locations
            .map(|i| match i {
                Instruction::Mov { src, dst } => Instruction::Mov {
                    src: self.pseudo_to_stack(src),
                    dst: self.pseudo_to_stack(dst),
                },
                Instruction::Unary(op, operand) => {
                    Instruction::Unary(op, self.pseudo_to_stack(operand))
                }
                _ => i,
            })
            // Second pass - fix mov instructions
            .flat_map(|i| match i {
                Instruction::Mov { src, dst } => match (src, dst) {
                    (Operand::Stack(src), Operand::Stack(dst)) => {
                        vec![
                            Instruction::Mov {
                                src: Operand::Stack(src),
                                dst: Operand::Reg(Register::R10),
                            },
                            Instruction::Mov {
                                src: Operand::Reg(Register::R10),
                                dst: Operand::Stack(dst),
                            },
                        ]
                    }
                    (src, dst) => vec![Instruction::Mov { src, dst }],
                },
                _ => vec![i],
            })
            .collect::<Vec<Instruction>>();

        if self.stack_offsets.len() > 0 {
            let mut with_stack_alloc =
                vec![Instruction::AllocateStack(self.stack_offsets.len() * 4)];
            with_stack_alloc.append(&mut out);
            with_stack_alloc
        } else {
            out
        }
    }

    fn pseudo_to_stack(&mut self, o: Operand) -> Operand {
        match o {
            Operand::PseudoReg(s) => {
                if let Some(o) = self.stack_offsets.get(&s) {
                    return o.clone();
                }

                let offset = ((self.stack_offsets.len() as i64) + 1) * 4;
                let operand = Operand::Stack(-offset);

                self.stack_offsets.insert(s, operand.clone());

                operand
            }
            _ => o,
        }
    }
}
