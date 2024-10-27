use std::fmt::Display;

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
    Not,
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Negate => {
                write!(f, "negl")
            }
            UnaryOperator::Not => {
                write!(f, "notl")
            }
        }
    }
}
