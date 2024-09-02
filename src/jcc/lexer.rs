use std::num::ParseIntError;

use crate::jcc::utf8::Utf8Reader;

use super::{utf8::Utf8ReadError, Token};

#[derive(thiserror::Error, Debug)]
pub enum LexerError {
    #[error("UTF-8 read error")]
    Utf8Error(#[from] Utf8ReadError),

    #[error("Unexpected EOF")]
    UnexpectedEOF,

    #[error("EOF")]
    EOF,

    #[error("Parse Int error")]
    ParseIntError(#[from] ParseIntError),
}

pub struct Lexer<R> {
    cur: Option<char>,
    reader: Utf8Reader<R>,
}

impl<R: std::io::Read> Lexer<R> {
    /// Construct a new Lexer over input from an object implementing the
    /// `std::io::Read` trait.  
    pub fn new(reader: R) -> Lexer<R> {
        Lexer {
            // seed with whitespace
            cur: Some(' '),
            reader: Utf8Reader::new(reader),
        }
    }

    /// Reads the next character
    fn read(&mut self) -> Result<char, LexerError> {
        if let Some(c) = self.reader.next() {
            self.cur = Some(c?);
            Ok(self.cur.unwrap())
        } else {
            self.cur = None;
            Err(LexerError::EOF)
        }
    }

    /// Peek at the current character without reading more data from the input
    /// stream
    fn peek(&mut self) -> Result<char, LexerError> {
        match self.cur {
            Some(c) => Ok(c),
            None => Err(LexerError::EOF),
        }
    }

    /// Clear the current character  
    fn consume(&mut self) {
        self.cur = Some(' ');
    }

    fn next_token(&mut self) -> Result<Token, LexerError> {
        let mut next = self.peek()?;

        while next.is_whitespace() {
            next = self.read()?;
        }

        match next {
            Token::PAREN_OPEN => {
                self.consume();
                Ok(Token::ParenOpen)
            }

            Token::PAREN_CLOSE => {
                self.consume();
                Ok(Token::ParenClose)
            }

            Token::BRACE_OPEN => {
                self.consume();
                Ok(Token::BraceOpen)
            }

            Token::BRACE_CLOSE => {
                self.consume();
                Ok(Token::BraceClose)
            }

            Token::SEMICOLON => {
                self.consume();
                Ok(Token::Semicolon)
            }

            _ => {
                let mut buf = String::new();
                while next.is_alphanumeric() {
                    buf.push(next);

                    next = self.read()?;
                }

                Token::new_from_string(buf)
            }
        }
    }
}

pub struct LexerIter<R> {
    lexer: Lexer<R>,
    error: bool,
}

impl<R: std::io::Read> Iterator for LexerIter<R> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.error {
            return None;
        }

        match self.lexer.next_token() {
            Ok(t) => Some(Ok(t)),
            Err(e) => {
                self.error = true;
                match e {
                    LexerError::EOF => None,
                    _ => Some(Err(e)),
                }
            }
        }
    }
}

impl<R: std::io::Read> IntoIterator for Lexer<R> {
    type Item = Result<Token, LexerError>;
    type IntoIter = LexerIter<R>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            lexer: self,
            error: false,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::jcc::{
        lexer::{Lexer, LexerError},
        Token,
    };

    #[test]
    fn test_simple() {
        test_lexer(
            "int main(void) { return 1; }",
            vec![
                Token::KeywordInt,
                Token::Identifier("main".to_owned()),
                Token::ParenOpen,
                Token::KeywordVoid,
                Token::ParenClose,
                Token::BraceOpen,
                Token::KeywordReturn,
                Token::ConstantInt(1),
                Token::Semicolon,
                Token::BraceClose,
            ],
        );
    }

    #[test]
    fn test_newlines() {
        test_lexer(
            "int main(void)\n{\n\treturn 1;\n}",
            vec![
                Token::KeywordInt,
                Token::Identifier("main".to_owned()),
                Token::ParenOpen,
                Token::KeywordVoid,
                Token::ParenClose,
                Token::BraceOpen,
                Token::KeywordReturn,
                Token::ConstantInt(1),
                Token::Semicolon,
                Token::BraceClose,
            ],
        )
    }

    #[test]
    fn test_error_bad_int() {
        test_lexer_error("int main(void)\n{\n\treturn 1foo;\n}")
    }

    #[test]
    fn test_error_bad_identifier() {
        test_lexer_error("int main(void)\n{\n\treturn @b;\n}")
    }

    fn test_lexer(prog: &str, expect: Vec<Token>) {
        let tokens = Lexer::new(prog.as_bytes())
            .into_iter()
            .collect::<Result<Vec<Token>, LexerError>>();
        assert_eq!(expect, tokens.unwrap());
    }

    fn test_lexer_error(prog: &str) {
        let tokens = Lexer::new(prog.as_bytes())
            .into_iter()
            .collect::<Result<Vec<Token>, LexerError>>();
        assert!(tokens.is_err());
    }
}
