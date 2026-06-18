use crate::error::{CompilerError, FormatError};
use std::borrow::Cow;
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

impl FormatError for InvalidParameterAmount {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Invocation Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        Cow::from(format!(
            "Expected {} parameters, {} were supplied.",
            self.expected, self.supplied
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.function.clone()]
    }
}
