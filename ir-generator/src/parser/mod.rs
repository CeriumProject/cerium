mod definition;
mod expression;
mod macros;

use crate::ast::{CeriumType, Qualifier, Script};
use crate::error::{CompilerError, CompilerResult, UnexpectedTokenError};
use crate::expect_token;
use crate::lexer::Lexer;
use crate::ranged::Ranged;
use crate::token::Token;
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
            definitions.push(definition?);
        }
        Ok(Script { definitions })
    }

    fn parse_qualifier(&mut self) -> CompilerResult<Ranged<Qualifier>> {
        let (range, initial_scope) =
            expect_token!(self.lexer, (range, Token::Ident(ident)), (range, ident))?;
        // while self.lexer.next_if(|token| matches!(token, Ok((_, Token::Scope)))) { todo!() }
        Ok((range, Qualifier::short(initial_scope)))
    }

    fn parse_type(&mut self) -> CompilerResult<Ranged<CeriumType>> {
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
                let return_type = if let Some(Ok((range, _))) =
                    self.lexer.next_if(|t| matches!(t, Ok((_, Token::Arrow))))
                {
                    end = *range.end();
                    Some(Box::new(self.parse_type()?.1))
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
