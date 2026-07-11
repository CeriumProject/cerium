use crate::ast::CeriumType;
use crate::error::{CompilerError, FormatError};
use crate::ranged::Ranged;
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct MismatchedBranchTypes {
    pub if_type: Ranged<CeriumType>,
    pub else_type: Ranged<CeriumType>,
}

impl From<MismatchedBranchTypes> for CompilerError {
    fn from(value: MismatchedBranchTypes) -> Self {
        CompilerError::MismatchesBranchTypes(value)
    }
}

impl FormatError for MismatchedBranchTypes {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Type Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        Cow::from(format!(
            "Branches of if-statement evaluate to different types '{}' and '{}'.",
            self.if_type.1, self.else_type.1
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.if_type.0.clone(), self.else_type.0.clone()]
    }
}
