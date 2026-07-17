use crate::ast::{Constant, Definition, Function, Structure};
use crate::error::{CompilerResult, UnexpectedTokenError};
use crate::parser::Parser;
use crate::token::Token;
use crate::{expect_token, next_matches};

impl Parser<'_> {
    pub(super) fn parse_definition(&mut self) -> Option<CompilerResult<Definition>> {
        match self.lexer.peek()? {
            Ok((_, Token::Fn)) => Some(self.parse_function()),
            Ok((_, Token::Const)) => Some(self.parse_constant()),
            Ok((_, Token::Struct)) => Some(self.parse_structure()),
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
        let mut generics = Vec::new();
        if next_matches!(self.lexer, Token::LessThan) {
            while !next_matches!(self.lexer, Token::GreaterThan) {
                generics.push(self.parse_qualifier()?);
                if next_matches!(self.lexer, Token::GreaterThan) {
                    break;
                }
                expect_token!(self.lexer, Token::Comma)?;
            }
        }
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
            generics,
            parameters,
            return_type,
            body,
        }))
    }

    fn parse_constant(&mut self) -> CompilerResult<Definition> {
        expect_token!(self.lexer, Token::Const)?;
        let name = self.parse_qualifier()?;
        expect_token!(self.lexer, Token::Colon)?;
        let r#type = self.parse_type()?;
        expect_token!(self.lexer, Token::Assign)?;
        let value = self.parse_expression()?;
        expect_token!(self.lexer, Token::Semicolon)?;
        Ok(Definition::Constant(Constant {
            name,
            r#type,
            value,
        }))
    }

    fn parse_structure(&mut self) -> CompilerResult<Definition> {
        expect_token!(self.lexer, Token::Struct)?;
        let name = self.parse_qualifier()?;
        expect_token!(self.lexer, Token::LBrace)?;
        let mut attributes = Vec::new();
        while !next_matches!(self.lexer, Token::RBrace) {
            let attribute = self.parse_qualifier()?;
            expect_token!(self.lexer, Token::Colon)?;
            let r#type = self.parse_type()?;
            attributes.push((attribute, r#type));
            if next_matches!(self.lexer, Token::RBrace) {
                break;
            }
            expect_token!(self.lexer, Token::Comma)?;
        }
        Ok(Definition::Structure(Structure { name, attributes }))
    }
}
