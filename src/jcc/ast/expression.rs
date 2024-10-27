use std::sync::atomic::{AtomicU64, Ordering};

use crate::jcc::ir::{self, Instruction, Value};

use super::{Constant, UnaryOperator};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Expression {
    kind: ExpressionKind,
}

impl Expression {
    pub fn new(kind: ExpressionKind) -> Expression {
        Expression { kind }
    }

    pub fn emit_ir(&self, instr: &mut Vec<Instruction>) -> Value {
        match &self.kind {
            ExpressionKind::Constant(c) => Value::Constant(c.value()),
            ExpressionKind::Unary(op, expr) => {
                let src = expr.emit_ir(instr);
                let dst = Value::Var(make_temp());
                let opr: ir::UnaryOperator = op.as_ir_op();

                instr.push(Instruction::Unary(opr, src, dst.clone()));

                dst
            }
        }
    }
}

#[derive(Debug)]
pub enum ExpressionKind {
    Constant(Constant),
    Unary(UnaryOperator, Box<Expression>),
}

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

fn make_temp() -> String {
    format!("tmp.{}", NEXT_ID.fetch_add(1, Ordering::Relaxed))
}
