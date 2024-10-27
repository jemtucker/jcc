use crate::jcc::utf8::Utf8Reader;

use super::{token::Type, Error, Token};

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
    fn read(&mut self) -> Result<char, Error> {
        if let Some(c) = self.reader.next() {
            self.cur = Some(c?);
            Ok(self.cur.unwrap())
        } else {
            self.cur = None;
            Err(Error::EOF)
        }
    }

    /// Peek at the current character without reading more data from the input
    /// stream
    fn peek(&mut self) -> Result<char, Error> {
        match self.cur {
            Some(c) => Ok(c),
            None => Err(Error::EOF),
        }
    }

    /// Clear the current character  
    fn consume(&mut self) {
        self.cur = Some(' ');
    }

    fn next_token(&mut self) -> Result<Token, Error> {
        let mut next = self.peek()?;

        while next.is_whitespace() {
            next = self.read()?;
        }

        match next {
            Token::PAREN_OPEN => {
                self.consume();
                Ok(Token::new(Type::ParenOpen, None))
            }

            Token::PAREN_CLOSE => {
                self.consume();
                Ok(Token::new(Type::ParenClose, None))
            }

            Token::BRACE_OPEN => {
                self.consume();
                Ok(Token::new(Type::BraceOpen, None))
            }

            Token::BRACE_CLOSE => {
                self.consume();
                Ok(Token::new(Type::BraceClose, None))
            }

            Token::SEMICOLON => {
                self.consume();
                Ok(Token::new(Type::Semicolon, None))
            }

            Token::TILDE => {
                self.consume();
                Ok(Token::new(Type::BitwiseComp, None))
            }

            Token::HYPHEN => {
                next = self.read()?;
                if next == Token::HYPHEN {
                    self.consume();
                    Ok(Token::new(Type::Decrement, None))
                } else {
                    Ok(Token::new(Type::Negation, None))
                }
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
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.error {
            return None;
        }

        match self.lexer.next_token() {
            Ok(t) => Some(Ok(t)),
            Err(e) => {
                self.error = true;
                match e {
                    Error::EOF => None,
                    _ => Some(Err(e)),
                }
            }
        }
    }
}

impl<R: std::io::Read> IntoIterator for Lexer<R> {
    type Item = Result<Token, Error>;
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
        lexer::{Error, Lexer},
        token::Type,
        Token,
    };

    #[test]
    fn test_simple() {
        test_lexer(
            "int main(void) { return 1; }",
            vec![
                Token::new(Type::Identifier, Some("int".to_owned())),
                Token::new(Type::Identifier, Some("main".to_owned())),
                Token::new(Type::ParenOpen, None),
                Token::new(Type::Identifier, Some("void".to_owned())),
                Token::new(Type::ParenClose, None),
                Token::new(Type::BraceOpen, None),
                Token::new(Type::Keyword, Some("return".to_owned())),
                Token::new(Type::Constant, Some("1".to_owned())),
                Token::new(Type::Semicolon, None),
                Token::new(Type::BraceClose, None),
            ],
        );
    }

    #[test]
    fn test_newlines() {
        test_lexer(
            "int main(void)\n{\n\treturn 1;\n}",
            vec![
                Token::new(Type::Identifier, Some("int".to_owned())),
                Token::new(Type::Identifier, Some("main".to_owned())),
                Token::new(Type::ParenOpen, None),
                Token::new(Type::Identifier, Some("void".to_owned())),
                Token::new(Type::ParenClose, None),
                Token::new(Type::BraceOpen, None),
                Token::new(Type::Keyword, Some("return".to_owned())),
                Token::new(Type::Constant, Some("1".to_owned())),
                Token::new(Type::Semicolon, None),
                Token::new(Type::BraceClose, None),
            ],
        )
    }

    #[test]
    fn test_bitwise_comp() {
        test_lexer(
            "int main(void)\n{\n\treturn ~1;\n}",
            vec![
                Token::new(Type::Identifier, Some("int".to_owned())),
                Token::new(Type::Identifier, Some("main".to_owned())),
                Token::new(Type::ParenOpen, None),
                Token::new(Type::Identifier, Some("void".to_owned())),
                Token::new(Type::ParenClose, None),
                Token::new(Type::BraceOpen, None),
                Token::new(Type::Keyword, Some("return".to_owned())),
                Token::new(Type::BitwiseComp, None),
                Token::new(Type::Constant, Some("1".to_owned())),
                Token::new(Type::Semicolon, None),
                Token::new(Type::BraceClose, None),
            ],
        )
    }

    #[test]
    fn test_negate() {
        test_lexer(
            "int main(void)\n{\n\treturn -1;\n}",
            vec![
                Token::new(Type::Identifier, Some("int".to_owned())),
                Token::new(Type::Identifier, Some("main".to_owned())),
                Token::new(Type::ParenOpen, None),
                Token::new(Type::Identifier, Some("void".to_owned())),
                Token::new(Type::ParenClose, None),
                Token::new(Type::BraceOpen, None),
                Token::new(Type::Keyword, Some("return".to_owned())),
                Token::new(Type::Negation, None),
                Token::new(Type::Constant, Some("1".to_owned())),
                Token::new(Type::Semicolon, None),
                Token::new(Type::BraceClose, None),
            ],
        )
    }

    #[test]
    fn test_decrement() {
        test_lexer(
            "int main(void)\n{\n\treturn 1--;\n}",
            vec![
                Token::new(Type::Identifier, Some("int".to_owned())),
                Token::new(Type::Identifier, Some("main".to_owned())),
                Token::new(Type::ParenOpen, None),
                Token::new(Type::Identifier, Some("void".to_owned())),
                Token::new(Type::ParenClose, None),
                Token::new(Type::BraceOpen, None),
                Token::new(Type::Keyword, Some("return".to_owned())),
                Token::new(Type::Constant, Some("1".to_owned())),
                Token::new(Type::Decrement, None),
                Token::new(Type::Semicolon, None),
                Token::new(Type::BraceClose, None),
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
            .collect::<Result<Vec<Token>, Error>>();
        assert_eq!(expect, tokens.unwrap());
    }

    fn test_lexer_error(prog: &str) {
        let tokens = Lexer::new(prog.as_bytes())
            .into_iter()
            .collect::<Result<Vec<Token>, Error>>();
        assert!(tokens.is_err());
    }
}
