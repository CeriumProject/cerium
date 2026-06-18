use crate::ast::CeriumType;
use crate::error::{CompilerError, FormatError};
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct InvalidCounterType {
    pub range: RangeInclusive<usize>,
    pub encountered: CeriumType,
}

impl From<InvalidCounterType> for CompilerError {
    fn from(error: InvalidCounterType) -> CompilerError {
        CompilerError::InvalidCounterType(error)
    }
}

impl FormatError for InvalidCounterType {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Invalid Counter Type")
    }

    fn error_explanation(&self) -> Cow<str> {
        let index_type = &self.encountered;
        Cow::from(format!(
            "Counter must be of integer or pointer type, not '{index_type}'."
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.range.clone()]
    }
}
