use crate::ast::{Definition, Function};
use crate::error::{CompilerResult, UnexpectedTokenError};
use crate::expect_token;
use crate::parser::Parser;
use crate::token::Token;

impl Parser<'_> {
    pub(super) fn parse_definition(&mut self) -> Option<CompilerResult<Definition>> {
        match self.lexer.peek()? {
            Ok((_, Token::Fn)) => Some(self.parse_function()),
            Ok((range, token)) => Some(Err(UnexpectedTokenError {
                token: token.clone(),
                range: range.clone(),
            }
            .into())),
            Err(err) => Some(Err(err.clone())),
        }
    }

    fn parse_function(&mut self) -> CompilerResult<Definition> {
        expect_token!(self.lexer, Token::Fn)?;
        let name = self.parse_qualifier()?;
        expect_token!(self.lexer, Token::LParen)?;
        let mut parameters = Vec::new();
        while !matches!(self.lexer.peek(), Some(Ok((_, Token::RParen)))) {
            let param_name = self.parse_qualifier()?;
            expect_token!(self.lexer, (_, Token::Colon), {})?;
            let param_type = self.parse_type()?;
            parameters.push((param_name, param_type));
            if self
                .lexer
                .next_if(|t| matches!(t, Ok((_, Token::Comma))))
                .is_none()
            {
                break;
            }
        }
        expect_token!(self.lexer, Token::RParen)?;
        let return_type = match self.lexer.next_if(|t| matches!(t, Ok((_, Token::Arrow)))) {
            Some(_) => Some(self.parse_type()?),
            None => None,
        };
        let body = self.parse_scope()?;

        Ok(Definition::Function(Function {
            name,
            parameters,
            return_type,
            body,
        }))
    }
}
