use crate::ast::CeriumType;
use crate::error::{CompilerError, FormatError};
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct MismatchedParameterType {
    pub parameter: RangeInclusive<usize>,
    pub expected: CeriumType,
    pub supplied: CeriumType,
}

impl From<MismatchedParameterType> for CompilerError {
    fn from(value: MismatchedParameterType) -> Self {
        CompilerError::MismatchedParameterType(value)
    }
}

impl FormatError for MismatchedParameterType {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Invocation Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        let expected = &self.expected;
        let supplied = &self.supplied;
        Cow::from(format!(
            "Parameter should be of type '{supplied}', not '{expected}'."
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.parameter.clone()]
    }
}
