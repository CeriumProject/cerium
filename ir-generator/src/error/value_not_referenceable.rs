use crate::error::{CompilerError, FormatError};
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct ValueNotReferenceable {
    pub range: RangeInclusive<usize>,
}

impl From<ValueNotReferenceable> for CompilerError {
    fn from(value: ValueNotReferenceable) -> Self {
        CompilerError::ValueNotReferenceable(value)
    }
}

impl FormatError for ValueNotReferenceable {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Referencing Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        Cow::from("Value is not referenceable. Consider storing it in a variable.")
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.range.clone()]
    }
}
