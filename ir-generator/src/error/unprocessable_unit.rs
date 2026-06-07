use crate::error::CompilerError;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct UnprocessableUnit {
    pub range: RangeInclusive<usize>,
}

impl From<UnprocessableUnit> for CompilerError {
    fn from(value: UnprocessableUnit) -> Self {
        CompilerError::UnprocessableUnit(value)
    }
}
