use crate::ast::CeriumType;
use crate::error::CompilerError;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct MismatchedParameterType {
    pub parameter: RangeInclusive<usize>,
    pub expected: CeriumType,
    pub supplied: CeriumType,
}

impl From<MismatchedParameterType> for CompilerError {
    fn from(value: MismatchedParameterType) -> Self {
        CompilerError::MismatchedParameterType(value)
    }
}
