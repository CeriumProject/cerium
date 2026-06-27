use crate::error::{CompilerError, FormatError};
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct UnparseableNumber {
    pub number: String,
    pub range: RangeInclusive<usize>,
}

impl From<UnparseableNumber> for CompilerError {
    fn from(value: UnparseableNumber) -> Self {
        CompilerError::UnparseableNumber(value)
    }
}

impl FormatError for UnparseableNumber {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Parsing Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        Cow::from(format!("Unable to parse constant '{}'.", &self.number))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.range.clone()]
    }
}
