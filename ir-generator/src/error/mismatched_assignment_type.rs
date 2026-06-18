use crate::ast::CeriumType;
use crate::error::{CompilerError, FormatError};
use crate::ranged::Ranged;
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct MismatchedAssignmentType {
    pub destination: Ranged<CeriumType>,
    pub source: Ranged<CeriumType>,
}

impl From<MismatchedAssignmentType> for CompilerError {
    fn from(error: MismatchedAssignmentType) -> CompilerError {
        CompilerError::MismatchedAssignmentType(error)
    }
}

impl FormatError for MismatchedAssignmentType {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Assignment Type Mismatch")
    }

    fn error_explanation(&self) -> Cow<str> {
        let dst = &self.destination.1;
        let src = &self.source.1;
        Cow::from(format!(
            "Destination has type '{dst}', value of type '{src}' was supplied."
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.destination.0.clone(), self.source.0.clone()]
    }
}
