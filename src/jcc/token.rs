use std::str::FromStr;

use super::Error;

#[allow(unused)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Keyword,
    Identifier,
    Constant,

    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,

    Semicolon,

    BitwiseComp,
    Negation,
    Decrement,
    Increment,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keyword => write!(f, "KEYWORD"),
            Self::Identifier => write!(f, "IDENTIFIER"),
            Self::Constant => write!(f, "CONSTANT"),
            Self::ParenOpen => write!(f, "("),
            Self::ParenClose => write!(f, ")"),
            Self::BraceOpen => write!(f, "{{"),
            Self::BraceClose => write!(f, "}}"),
            Self::Semicolon => write!(f, ";"),
            Self::BitwiseComp => write!(f, "~"),
            Self::Negation => write!(f, "-"),
            Self::Decrement => write!(f, "--"),
            Self::Increment => write!(f, "++"),
        }
    }
}

#[derive(Debug)]
pub struct TypeVec(pub Vec<Type>);

impl std::fmt::Display for TypeVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut more_than_one = false;
        for typ in &self.0 {
            if more_than_one {
                write!(f, "|")?;
            } else {
                more_than_one = true;
            }

            typ.fmt(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: Type,
    value: Option<String>,
}

impl Token {
    pub(crate) const KEYWORD_RETURN: &'static str = "return";

    pub(crate) const PAREN_OPEN: char = '(';
    pub(crate) const PAREN_CLOSE: char = ')';
    pub(crate) const BRACE_OPEN: char = '{';
    pub(crate) const BRACE_CLOSE: char = '}';
    pub(crate) const SEMICOLON: char = ';';
    pub(crate) const TILDE: char = '~';
    pub(crate) const HYPHEN: char = '-';

    /// Create a new token with the specified type and value
    pub fn new(token_type: Type, value: Option<String>) -> Token {
        Token { token_type, value }
    }

    /// new_from_string parses a string into a keyword, identifier or constant
    /// type token.
    pub fn new_from_string(s: String) -> Result<Token, Error> {
        let Some(first) = s.chars().next() else {
            return Err(Error::UnexpectedEOF);
        };

        if first.is_numeric() {
            if !s.chars().all(|c| c.is_numeric()) {
                return Err(Error::InvalidToken(s));
            }
            return Self::new_constant(s);
        }

        match s.as_str() {
            // TODO throwing away work here - be better?
            Token::KEYWORD_RETURN => Ok(Self::new(Type::Keyword, Some(s))),
            _ => Ok(Self::new(Type::Identifier, Some(s))),
        }
    }

    /// Construct a new Constant type Token
    pub fn new_constant(s: String) -> Result<Token, Error> {
        Ok(Self::new(Type::Constant, Some(s)))
    }

    /// Return the Type of this token
    pub fn token_type(&self) -> Type {
        self.token_type
    }

    pub fn value(&self) -> Result<&str, Error> {
        match &self.value {
            Some(v) => Ok(v),
            None => Err(Error::InvalidToken("".to_owned())),
        }
    }

    /// Parses this tokens value to return a value of the type `F`.
    pub fn parse_value<F: FromStr>(&self) -> Result<F, Error> {
        match &self.value {
            Some(v) => match FromStr::from_str(v) {
                Ok(r) => Ok(r),
                Err(_) => Err(Error::ParseTokenError(v.to_owned())),
            },
            None => Err(Error::InvalidToken("".to_owned())),
        }
    }
}
