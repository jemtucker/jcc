use super::{token::Type, utf8::Utf8ReadError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    //
    // Lexing errors
    //
    #[error("UTF-8 read error")]
    Utf8Error(#[from] Utf8ReadError),

    #[error("Unexpected EOF")]
    UnexpectedEOF,

    #[error("EOF")]
    EOF,

    //
    // Parsing errors
    //
    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("Unexpected token: expected {exp} but got {got}")]
    UnexpectedToken { exp: Type, got: Type },

    #[error("Unexpected token: expected EOF but got {0}")]
    UnexpectedExtraToken(Type),

    #[error("Error parsing token: {0}")]
    ParseTokenError(String),
}
