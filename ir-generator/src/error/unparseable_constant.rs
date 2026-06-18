use crate::error::{CompilerError, FormatError};
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct UnparseableConstant {
    pub raw_constant: String,
    pub range: RangeInclusive<usize>,
}

impl From<UnparseableConstant> for CompilerError {
    fn from(value: UnparseableConstant) -> Self {
        CompilerError::UnparseableConstant(value)
    }
}

impl FormatError for UnparseableConstant {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Parsing Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        Cow::from(format!(
            "Unable to parse constant '{}'.",
            &self.raw_constant
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.range.clone()]
    }
}
