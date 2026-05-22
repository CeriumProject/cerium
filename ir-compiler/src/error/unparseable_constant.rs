use std::ops::RangeInclusive;
use crate::error::CompilerError;

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
