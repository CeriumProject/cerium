use crate::error::{CompilerError, FormatError};
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct InvalidGenericAmount {
    pub range: RangeInclusive<usize>,
    pub expected: usize,
    pub supplied: usize,
}

impl From<InvalidGenericAmount> for CompilerError {
    fn from(error: InvalidGenericAmount) -> Self {
        CompilerError::InvalidGenericAmount(error)
    }
}

impl FormatError for InvalidGenericAmount {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Generics Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        Cow::from(format!(
            "Expected {0} generic types. {1} were supplied.",
            self.expected, self.supplied
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.range.clone()]
    }
}
