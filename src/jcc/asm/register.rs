use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Register {
    AX,
    R10,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AX => write!(f, "%eax"),
            Self::R10 => write!(f, "%r10d"),
        }
    }
}
