use std::ops::RangeInclusive;
use crate::error::CompilerError;

#[derive(Clone, Debug, PartialEq)]
pub struct UnprocessableUnit {
    pub range: RangeInclusive<usize>,
}

impl From<UnprocessableUnit> for CompilerError {
    fn from(value: UnprocessableUnit) -> Self {
        CompilerError::UnprocessableUnit(value)
    }
}