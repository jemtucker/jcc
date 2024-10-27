use super::{
    ast::{
        Constant, Expression, ExpressionKind, Function, Program, Statement, UnaryOperator,
        UnaryOperatorKind,
    },
    token::{Type, TypeVec},
    Error, Token,
};

pub struct Parser<I>
where
    I: Iterator<Item = Result<Token, Error>>,
{
    tokens: I,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Result<Token, Error>>,
{
    /// Construct a new parser for an iterator of tokens
    pub fn new(tokens: I) -> Parser<I> {
        Parser { tokens }
    }

    /// Parse the tokens
    pub fn parse(&mut self) -> Result<Program, Error> {
        self.parse_program()
    }

    fn parse_program(&mut self) -> Result<Program, Error> {
        let program = Program::new(self.parse_function()?);
        self.expect_eof()?;

        Ok(program)
    }

    fn parse_function(&mut self) -> Result<Function, Error> {
        let _return_type = self.expect(Type::Identifier)?;
        let name = self.expect(Type::Identifier)?;

        // Arguments
        self.expect(Type::ParenOpen)?;
        let _arg = self.expect(Type::Identifier)?;
        self.expect(Type::ParenClose)?;

        // Body
        self.expect(Type::BraceOpen)?;
        let statement = self.parse_statement()?;
        self.expect(Type::BraceClose)?;

        Ok(Function::new(name.value()?, statement))
    }

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        let token = self.expect(Type::Keyword)?;
        let value = token.value()?;
        if value != "return" {
            return Err(Error::InvalidToken(value.to_owned()));
        }

        let expr = self.parse_expression()?;

        let statement = Statement::new(expr);
        self.expect(Type::Semicolon)?;

        Ok(statement)
    }

    fn parse_expression(&mut self) -> Result<Expression, Error> {
        let next = self.next_token()?;
        match next.token_type() {
            Type::ParenOpen => {
                let expr = self.parse_expression()?;
                self.expect(Type::ParenClose)?;
                Ok(expr)
            }

            Type::Constant => self.parse_expression_constant(next),

            Type::BitwiseComp | Type::Negation | Type::Decrement => {
                self.parse_expression_unary(next)
            }

            _ => Err(Error::UnexpectedToken {
                exp: TypeVec(vec![
                    Type::Constant,
                    Type::BitwiseComp,
                    Type::Negation,
                    Type::Decrement,
                ]),
                got: next.token_type(),
            }),
        }
    }

    fn parse_expression_unary(&mut self, token: Token) -> Result<Expression, Error> {
        let operator = match token.token_type() {
            Type::BitwiseComp => UnaryOperatorKind::Complement,
            Type::Negation => UnaryOperatorKind::Negate,

            _ => {
                return Err(Error::UnexpectedToken {
                    exp: TypeVec(vec![Type::BitwiseComp, Type::Negation, Type::Decrement]),
                    got: token.token_type(),
                })
            }
        };

        Ok(Expression::new(ExpressionKind::Unary(
            UnaryOperator::new(operator),
            Box::new(self.parse_expression()?),
        )))
    }

    fn parse_expression_constant(&mut self, token: Token) -> Result<Expression, Error> {
        Ok(Expression::new(ExpressionKind::Constant(
            self.parse_constant(token)?,
        )))
    }

    fn parse_constant(&mut self, token: Token) -> Result<Constant, Error> {
        Ok(Constant::new(token.parse_value()?))
    }

    /// Return the next token from the input stream.
    ///
    /// If we reach the end of the feed `Error::UnexpectedEOF` is returned.
    fn next_token(&mut self) -> Result<Token, Error> {
        let Some(next) = self.tokens.next() else {
            return Err(Error::UnexpectedEOF);
        };

        next
    }

    /// Return the next token from the input stream, if it matches the expected
    /// type, otherwise return `Error::UnexpectedToken.
    ///
    /// If we reach the end of the feed `Error::UnexpectedEOF` is returned.
    fn expect(&mut self, expected: Type) -> Result<Token, Error> {
        let token = self.next_token()?;

        if token.token_type() != expected {
            return Err(Error::UnexpectedToken {
                exp: TypeVec(vec![expected]),
                got: token.token_type(),
            });
        }

        Ok(token)
    }

    fn expect_eof(&mut self) -> Result<(), Error> {
        match self.tokens.next() {
            Some(token) => Err(Error::UnexpectedExtraToken(token?.token_type())),
            None => Ok(()),
        }
    }
}
