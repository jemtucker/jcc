use std::{collections::HashMap, vec};

use crate::jcc::asm::{Instruction, Operand, Register};

use super::{
    asm::{self},
    ast::Program,
    ir,
};

pub struct CodeGenerator {}

impl CodeGenerator {
    pub fn new() -> CodeGenerator {
        CodeGenerator {}
    }

    pub fn codegen(&self, program: Program) -> asm::Program {
        self.stage_asm(self.stage_ir(program))
    }

    fn stage_ir(&self, program: Program) -> ir::Program {
        program.into()
    }

    fn stage_asm(&self, program: ir::Program) -> asm::Program {
        self.remove_pseudo_registers(program.into())
    }

    /// Convert all Pseudo registers into Stack locations and ensure mov operations
    /// are updated accordingly
    fn remove_pseudo_registers(&self, program: asm::Program) -> asm::Program {
        program
            .map(|f| {
                let mut tracker = StackTracker::default();
                f.map(|i| match i {
                    Instruction::Mov { src, dst } => Instruction::Mov {
                        src: tracker.pseudo_to_stack(src),
                        dst: tracker.pseudo_to_stack(dst),
                    },
                    Instruction::Unary(op, operand) => {
                        Instruction::Unary(op, tracker.pseudo_to_stack(operand))
                    }
                    _ => i,
                })
                .prepend(tracker.allocate_stack_preamble())
            })
            .map(|f| {
                f.flat_map(|i| match i {
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
            })
    }
}

#[derive(Default)]
struct StackTracker {
    stack_offsets: HashMap<String, Operand>,
}

impl StackTracker {
    /// Map a pseudo register to a new stack location, using known locations when
    /// we've seen the same register in use already (as this indicates its the
    /// same data).  
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

    /// Returns the required function preamble needed to allocate the amount of
    /// stack required for the addresses we've tracked.
    fn allocate_stack_preamble(&self) -> Vec<Instruction> {
        let alloc = self.stack_offsets.len() * 4;

        if alloc > 0 {
            vec![Instruction::AllocateStack(alloc)]
        } else {
            vec![]
        }
    }
}
