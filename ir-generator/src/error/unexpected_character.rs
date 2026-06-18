use crate::error::{CompilerError, FormatError};
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct UnexpectedCharacterError {
    pub character: char,
    pub idx: usize,
}

impl From<UnexpectedCharacterError> for CompilerError {
    fn from(value: UnexpectedCharacterError) -> Self {
        CompilerError::UnexpectedCharacterError(value)
    }
}

impl FormatError for UnexpectedCharacterError {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Lexing Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        Cow::from(format!(
            "Encountered unexpected character '{}'",
            &self.character
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.idx..=self.idx]
    }
}
