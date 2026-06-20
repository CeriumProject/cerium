use crate::ast::{CeriumType, Qualifier};
use crate::error::{CompilerError, FormatError};
use crate::ranged::Ranged;
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct FalseFieldType {
    pub field: Ranged<Qualifier>,
    pub expected: CeriumType,
    pub actual: CeriumType,
}

impl From<FalseFieldType> for CompilerError {
    fn from(err: FalseFieldType) -> Self {
        CompilerError::FalseFieldType(err)
    }
}

impl FormatError for FalseFieldType {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Struct Initialization Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        Cow::from(format!(
            "Attribute '{}' has type '{}'. Tried to assign value of type '{}'.",
            &self.field.1, &self.expected, &self.actual,
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.field.0.clone()]
    }
}
