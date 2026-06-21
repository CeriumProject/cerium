mod definition;
mod expression;
mod macros;

use crate::ast::{CeriumType, Qualifier, Script};
use crate::error::{CompilerError, CompilerResult, UnexpectedTokenError};
use crate::lexer::Lexer;
use crate::ranged::Ranged;
use crate::token::Token;
use crate::{expect_token, next_matches};
use std::iter::Peekable;
use std::ops::RangeInclusive;

fn join_ranges<Lhs, Rhs>(lhs: &Ranged<Lhs>, rhs: &Ranged<Rhs>) -> RangeInclusive<usize> {
    *lhs.0.start()..=*rhs.0.end()
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser {
            lexer: lexer.peekable(),
        }
    }

    pub fn parse(&mut self) -> CompilerResult<Script> {
        let mut definitions = Vec::new();
        while let Some(definition) = self.parse_definition() {
            definitions.push(definition?.optimize());
        }
        Ok(Script { definitions })
    }

    fn parse_qualifier(&mut self) -> CompilerResult<Ranged<Qualifier>> {
        let (start, mut end, mut scopes) = expect_token!(
            self.lexer,
            (range, Token::Ident(ident)),
            (*range.start(), *range.end(), vec![ident])
        )?;
        while next_matches!(self.lexer, Token::Scope) {
            expect_token!(self.lexer, (range, Token::Ident(ident)), {
                end = *range.end();
                scopes.push(ident);
            })?;
        }
        Ok((start..=end, Qualifier::new(scopes)))
    }

    fn parse_type(&mut self) -> CompilerResult<Ranged<CeriumType>> {
        if matches!(self.lexer.peek(), Some(Ok((_, Token::Ident(_))))) {
            let (range, name) = self.parse_qualifier()?;
            return Ok((range, CeriumType::Struct(name)));
        }
        match expect_token!(self.lexer, token, token)? {
            (range, Token::I16) => Ok((range, CeriumType::I16)),
            (range, Token::U16) => Ok((range, CeriumType::U16)),
            (range, Token::F16) => Ok((range, CeriumType::F16)),
            (start_range, Token::Ampersand) => {
                let (end_range, inner_type) = self.parse_type()?;
                Ok((
                    *start_range.start()..=*end_range.end(),
                    CeriumType::Reference(Box::new(inner_type)),
                ))
            }
            // TODO: refactor ts
            (start_range, Token::Fn) => {
                expect_token!(self.lexer, (_, Token::LParen), {})?;
                let mut param_types = Vec::new();
                while !matches!(self.lexer.peek(), Some(Ok((_, Token::RParen)))) {
                    let (_, param_type) = self.parse_type()?;
                    param_types.push(param_type);
                    if self
                        .lexer
                        .next_if(|t| matches!(t, Ok((_, Token::Comma))))
                        .is_none()
                    {
                        break;
                    }
                }
                let mut end =
                    expect_token!(self.lexer, (end_range, Token::RParen), *end_range.end())?;
                let return_type = if let Some(Ok(_)) =
                    self.lexer.next_if(|t| matches!(t, Ok((_, Token::Arrow))))
                {
                    let (range, return_type) = self.parse_type()?;
                    end = *range.end();
                    Some(Box::new(return_type))
                } else {
                    None
                };
                Ok((
                    *start_range.start()..=end,
                    CeriumType::Function(param_types, return_type),
                ))
            }
            (range, token) => Err(CompilerError::UnexpectedTokenError(UnexpectedTokenError {
                range,
                token,
            })),
        }
    }
}
