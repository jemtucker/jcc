use super::lexer::LexerError;

#[derive(Debug, PartialEq)]
pub enum Token {
    KeywordInt,
    KeywordVoid,
    KeywordReturn,

    Identifier(String),
    ConstantString(String),
    ConstantInt(u64),

    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,

    Semicolon,
}

impl Token {
    pub(crate) const KEYWORD_INT: &'static str = "int";
    pub(crate) const KEYWORD_VOID: &'static str = "void";
    pub(crate) const KEYWORD_RETURN: &'static str = "return";

    pub(crate) const PAREN_OPEN: char = '(';
    pub(crate) const PAREN_CLOSE: char = ')';
    pub(crate) const BRACE_OPEN: char = '{';
    pub(crate) const BRACE_CLOSE: char = '}';
    pub(crate) const SEMICOLON: char = ';';

    /// new_from_string parses a string into a keyword, identifier or constant
    /// type token.
    pub fn new_from_string(s: String) -> Result<Token, LexerError> {
        let Some(first) = s.chars().next() else {
            return Err(LexerError::UnexpectedEOF);
        };

        if first.is_numeric() {
            return Self::new_from_integer_string(s);
        }

        match s.as_str() {
            Token::KEYWORD_INT => Ok(Token::KeywordInt),
            Token::KEYWORD_VOID => Ok(Token::KeywordVoid),
            Token::KEYWORD_RETURN => Ok(Token::KeywordReturn),
            _ => Ok(Token::Identifier(s)),
        }
    }

    pub fn new_from_integer_string(s: String) -> Result<Token, LexerError> {
        Ok(Token::ConstantInt(s.parse::<u64>()?))
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KeywordInt => write!(f, "Keyword(int)"),
            Self::KeywordVoid => write!(f, "Keyword(void)"),
            Self::KeywordReturn => write!(f, "Keyword(return)"),
            Self::Identifier(s) => write!(f, "Identifier('{}')", s),
            Self::ConstantInt(i) => write!(f, "Constant({})", i),
            Self::ConstantString(s) => write!(f, "Constant('{}')", s),
            Self::ParenOpen => write!(f, "("),
            Self::ParenClose => write!(f, ")"),
            Self::BraceOpen => write!(f, "{{"),
            Self::BraceClose => write!(f, "}}"),
            Self::Semicolon => write!(f, ";"),
        }
    }
}
