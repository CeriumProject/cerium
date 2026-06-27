use crate::error::{CompilerError, FormatError};
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct InvalidCharLength {
    pub range: RangeInclusive<usize>,
    pub encountered: usize,
}

impl From<InvalidCharLength> for CompilerError {
    fn from(error: InvalidCharLength) -> Self {
        CompilerError::InvalidCharLength(error)
    }
}

impl FormatError for InvalidCharLength {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Char Literal Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        let length = self.encountered;
        Cow::from(format!(
            "Char literals must have length 1. Has length {length}."
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.range.clone()]
    }
}
