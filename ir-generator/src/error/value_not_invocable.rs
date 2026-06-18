use crate::ast::CeriumType;
use crate::error::{CompilerError, FormatError};
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct ValueNotInvocable {
    pub range: RangeInclusive<usize>,
    pub r#type: CeriumType,
}

impl From<ValueNotInvocable> for CompilerError {
    fn from(value: ValueNotInvocable) -> Self {
        CompilerError::ValueNotInvocable(value)
    }
}

impl FormatError for ValueNotInvocable {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Invocation Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        Cow::from(format!(
            "Cannot invoke values of type '{}'. Must be function.",
            &self.r#type
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.range.clone()]
    }
}
