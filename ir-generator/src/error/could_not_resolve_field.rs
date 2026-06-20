use crate::ast::{CeriumType, Qualifier};
use crate::error::{CompilerError, FormatError};
use crate::ranged::Ranged;
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct CouldNotResolveField {
    pub maybe_struct_type: CeriumType,
    pub name: Ranged<Qualifier>,
}

impl From<CouldNotResolveField> for CompilerError {
    fn from(value: CouldNotResolveField) -> Self {
        CompilerError::CouldNotResolveField(value)
    }
}

impl FormatError for CouldNotResolveField {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Field Resolving Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        let name = &self.name.1;
        let r#type = &self.maybe_struct_type;
        Cow::from(format!(
            "Could not find field '{name}' on value of type {}.",
            r#type
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.name.0.clone()]
    }
}
