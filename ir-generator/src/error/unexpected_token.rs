use crate::error::CompilerError;
use crate::token::Token;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct UnexpectedTokenError {
    pub token: Token,
    pub range: RangeInclusive<usize>,
}

impl Into<CompilerError> for UnexpectedTokenError {
    fn into(self) -> CompilerError {
        CompilerError::UnexpectedTokenError(self)
    }
}
