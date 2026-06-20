use crate::ast::Qualifier;
use crate::error::{CompilerError, FormatError};
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct UnassignedField {
    pub range: RangeInclusive<usize>,
    pub field: Qualifier,
}

impl From<UnassignedField> for CompilerError {
    fn from(value: UnassignedField) -> Self {
        CompilerError::UnassignedField(value)
    }
}

impl FormatError for UnassignedField {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Struct Initialization Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        Cow::from(format!("Field '{}' is not initialized.", &self.field))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.range.clone()]
    }
}
