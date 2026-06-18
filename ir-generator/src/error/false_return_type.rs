use crate::ast::{CeriumType, Qualifier};
use crate::error::{CompilerError, FormatError};
use crate::ranged::Ranged;
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct FalseReturnType {
    pub function: Ranged<Qualifier>,
    pub expected: Option<Ranged<CeriumType>>,
    pub actual: Option<Ranged<CeriumType>>,
}

impl From<FalseReturnType> for CompilerError {
    fn from(err: FalseReturnType) -> Self {
        CompilerError::FalseReturnType(err)
    }
}

impl FormatError for FalseReturnType {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Wrong Return Type")
    }

    fn error_explanation(&self) -> Cow<str> {
        let expected = self
            .expected
            .as_ref()
            .map(|(_, t)| t.to_string())
            .unwrap_or_else(|| String::from("unit"));
        let actual = self
            .actual
            .as_ref()
            .map(|(_, t)| t.to_string())
            .unwrap_or_else(|| String::from("unit"));
        let function = &self.function.1;
        Cow::from(format!(
            "Expected return type '{expected}' for function '{function}', found '{actual}'."
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        let mut result = Vec::new();
        if let Some((range, _)) = &self.expected {
            result.push(range.clone());
        }
        if let Some((range, _)) = &self.actual {
            result.push(range.clone());
        }
        result
    }
}
