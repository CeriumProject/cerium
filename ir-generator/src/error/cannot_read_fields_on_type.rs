use crate::ast::CeriumType;
use crate::error::{CompilerError, FormatError};
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct CannotReadFieldsOnType {
    pub range: RangeInclusive<usize>,
    pub r#type: CeriumType,
}

impl From<CannotReadFieldsOnType> for CompilerError {
    fn from(value: CannotReadFieldsOnType) -> Self {
        CompilerError::CannotReadFieldsOnType(value)
    }
}

impl FormatError for CannotReadFieldsOnType {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Field Resolving Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        Cow::from(format!(
            "Can only access attributes on references to structs. Encountered value of type '{}'.",
            &self.r#type
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.range.clone()]
    }
}
