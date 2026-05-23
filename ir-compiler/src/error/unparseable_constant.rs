use crate::error::CompilerError;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct UnparseableConstant {
    pub raw_constant: String,
    pub range: RangeInclusive<usize>,
}

impl Into<CompilerError> for UnparseableConstant {
    fn into(self) -> CompilerError {
        CompilerError::UnparseableConstant(self)
    }
}
