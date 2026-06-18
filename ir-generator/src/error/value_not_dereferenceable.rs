use crate::ast::CeriumType;
use crate::error::{CompilerError, FormatError};
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct ValueNotDereferenceable {
    pub range: RangeInclusive<usize>,
    pub r#type: CeriumType,
}

impl From<ValueNotDereferenceable> for CompilerError {
    fn from(err: ValueNotDereferenceable) -> Self {
        CompilerError::ValueNotDereferenceable(err)
    }
}

impl FormatError for ValueNotDereferenceable {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Dereferencing Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        let r#type = &self.r#type;
        Cow::from(format!(
            "Cannot dereference values of type '{type}'. Must be reference."
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.range.clone()]
    }
}
