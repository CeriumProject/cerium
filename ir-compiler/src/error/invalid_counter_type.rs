use crate::ast::CeriumType;
use crate::error::CompilerError;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct InvalidCounterType {
    pub range: RangeInclusive<usize>,
    pub encountered: CeriumType,
}

impl From<InvalidCounterType> for CompilerError {
    fn from(error: InvalidCounterType) -> CompilerError {
        CompilerError::InvalidCounterType(error)
    }
}
