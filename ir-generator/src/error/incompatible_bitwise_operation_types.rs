use crate::ast::CeriumType;
use crate::error::{CompilerError, FormatError};
use crate::ranged::Ranged;
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct IncompatibleBitwiseOperationTypes {
    pub lhs: Ranged<CeriumType>,
    pub rhs: Ranged<CeriumType>,
}

impl From<IncompatibleBitwiseOperationTypes> for CompilerError {
    fn from(error: IncompatibleBitwiseOperationTypes) -> Self {
        CompilerError::IncompatibleBitwiseOperationTypes(error)
    }
}

impl FormatError for IncompatibleBitwiseOperationTypes {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Operation Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        Cow::from(format!(
            "Operands for bitwise operations must be 'u16'. '{}' and '{}' were given.",
            &self.lhs.1, &self.rhs.1,
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.lhs.0.clone(), self.rhs.0.clone()]
    }
}
