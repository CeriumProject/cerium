use crate::ast::CeriumType;
use crate::error::{CompilerError, FormatError};
use crate::ranged::Ranged;
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct IncompatibleTypes {
    pub lhs: Ranged<CeriumType>,
    pub rhs: Ranged<CeriumType>,
}

impl From<IncompatibleTypes> for CompilerError {
    fn from(e: IncompatibleTypes) -> Self {
        CompilerError::IncompatibleTypes(e)
    }
}

impl FormatError for IncompatibleTypes {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Operation Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        let lhs = &self.lhs.1;
        let rhs = &self.rhs.1;
        Cow::from(format!(
            "Values types '{lhs}' and '{rhs}' are incompatible for given operation."
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.lhs.0.clone(), self.rhs.0.clone()]
    }
}
