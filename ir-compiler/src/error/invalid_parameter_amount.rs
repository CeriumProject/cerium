use crate::error::CompilerError;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct InvalidParameterAmount {
    pub function: RangeInclusive<usize>,
    pub expected: usize,
    pub supplied: usize,
}

impl From<InvalidParameterAmount> for CompilerError {
    fn from(value: InvalidParameterAmount) -> Self {
        CompilerError::InvalidParameterAmount(value)
    }
}
