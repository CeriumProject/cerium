use crate::error::{CompilerError, FormatError};
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct UnexpectedEof;

impl From<UnexpectedEof> for CompilerError {
    fn from(value: UnexpectedEof) -> Self {
        CompilerError::UnexpectedEof(value)
    }
}

impl FormatError for UnexpectedEof {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Parsing Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        Cow::from("Encountered abrupt end while parsing code.")
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![]
    }
}
