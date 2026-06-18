use crate::ast::Qualifier;
use crate::error::{CompilerError, FormatError};
use crate::ranged::Ranged;
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct CouldNotResolveVariable {
    pub(crate) name: Ranged<Qualifier>,
}

impl From<CouldNotResolveVariable> for CompilerError {
    fn from(value: CouldNotResolveVariable) -> Self {
        CompilerError::CouldNotResolveVariable(value)
    }
}

impl FormatError for CouldNotResolveVariable {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Variable Resolving Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        let name = &self.name.1;
        Cow::from(format!("Could not find variable '{name}'."))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.name.0.clone()]
    }
}
