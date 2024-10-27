use crate::jcc::ir::Instruction;

use super::expression::Expression;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Statement {
    kind: StatementKind,
}

impl Statement {
    pub fn new(expr: Expression) -> Statement {
        Statement {
            kind: StatementKind::Return(expr),
        }
    }
}

impl Into<Vec<Instruction>> for Statement {
    fn into(self) -> Vec<Instruction> {
        self.kind.into()
    }
}

#[derive(Debug)]
pub enum StatementKind {
    Return(Expression),
}

impl Into<Vec<Instruction>> for StatementKind {
    fn into(self) -> Vec<Instruction> {
        match self {
            Self::Return(e) => {
                let mut instr = Vec::<Instruction>::default();
                let dst = e.emit_ir(&mut instr);

                instr.push(Instruction::Return(dst));

                instr
            }
        }
    }
}
