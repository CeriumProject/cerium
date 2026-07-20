use crate::ast::CeriumType;
use crate::error::{CompilerError, FormatError};
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct InvalidTurbofishType {
    pub range: RangeInclusive<usize>,
    pub r#type: CeriumType,
}

impl From<InvalidTurbofishType> for CompilerError {
    fn from(error: InvalidTurbofishType) -> Self {
        CompilerError::InvalidTurbofishType(error)
    }
}

impl FormatError for InvalidTurbofishType {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Generics Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        Cow::from(format!(
            "Turbofish can only be used on generic function pointers. Type '{0}' was given.",
            &self.r#type
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.range.clone()]
    }
}
