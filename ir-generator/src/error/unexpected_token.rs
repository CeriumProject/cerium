use crate::error::{CompilerError, FormatError};
use crate::token::Token;
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct UnexpectedTokenError {
    pub token: Token,
    pub range: RangeInclusive<usize>,
}

impl From<UnexpectedTokenError> for CompilerError {
    fn from(value: UnexpectedTokenError) -> Self {
        CompilerError::UnexpectedTokenError(value)
    }
}

impl FormatError for UnexpectedTokenError {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Parsing Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        Cow::from(format!("Encountered unexpected token '{:?}'", &self.token))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.range.clone()]
    }
}
