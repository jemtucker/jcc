use super::{
    ast::{Constant, Expression, Function, Program, Statement},
    token::Type,
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

        let expr = Expression::new(self.parse_constant()?);
        let statement = Statement::new(expr);
        self.expect(Type::Semicolon)?;

        Ok(statement)
    }

    fn parse_constant(&mut self) -> Result<Constant, Error> {
        Ok(Constant::new(self.expect(Type::Constant)?.parse_value()?))
    }

    fn expect(&mut self, expected: Type) -> Result<Token, Error> {
        let Some(next) = self.tokens.next() else {
            return Err(Error::UnexpectedEOF);
        };

        let token = next?;

        if token.token_type() != expected {
            return Err(Error::UnexpectedToken {
                exp: expected,
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
