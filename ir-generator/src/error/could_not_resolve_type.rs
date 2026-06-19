use crate::ast::Qualifier;
use crate::error::{CompilerError, FormatError};
use crate::ranged::Ranged;
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct CouldNotResolveType {
    pub(crate) name: Ranged<Qualifier>,
}

impl From<CouldNotResolveType> for CompilerError {
    fn from(value: CouldNotResolveType) -> Self {
        CompilerError::CouldNotResolveType(value)
    }
}

impl FormatError for CouldNotResolveType {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Type Resolving Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        let name = &self.name.1;
        Cow::from(format!("Could not find type '{name}'."))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.name.0.clone()]
    }
}
