use crate::ast::CeriumType;
use crate::error::{CompilerError, FormatError};
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct ConditionMustBeBool {
    pub condition_range: RangeInclusive<usize>,
    pub encountered_type: CeriumType,
}

impl From<ConditionMustBeBool> for CompilerError {
    fn from(condition: ConditionMustBeBool) -> Self {
        CompilerError::ConditionMustBeBool(condition)
    }
}

impl FormatError for ConditionMustBeBool {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Type Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        Cow::from(format!(
            "Type of condition must be 'bool'. Encountered '{}'.",
            self.encountered_type
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.condition_range.clone()]
    }
}
