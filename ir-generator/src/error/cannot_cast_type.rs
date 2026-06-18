use crate::ast::CeriumType;
use crate::error::{CompilerError, FormatError};
use crate::ranged::Ranged;
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct CannotCastType {
    pub from: Ranged<CeriumType>,
    pub to: Ranged<CeriumType>,
}

impl From<CannotCastType> for CompilerError {
    fn from(value: CannotCastType) -> CompilerError {
        CompilerError::CannotCastType(value)
    }
}

impl FormatError for CannotCastType {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Casting Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        let from_type = &self.from.1;
        let to_type = &self.to.1;
        Cow::from(format!(
            "Cannot cast value of type '{from_type}' to type '{to_type}'."
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.from.0.clone(), self.to.0.clone()]
    }
}
