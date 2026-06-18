use crate::ast::CeriumType;
use crate::error::{CompilerError, FormatError};
use crate::ranged::Ranged;
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct TypeAliasHasDifferentSize {
    pub source: Ranged<CeriumType>,
    pub target: Ranged<CeriumType>,
}

impl From<TypeAliasHasDifferentSize> for CompilerError {
    fn from(e: TypeAliasHasDifferentSize) -> Self {
        CompilerError::TypeAliasHasDifferentSize(e)
    }
}

impl FormatError for TypeAliasHasDifferentSize {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Aliasing Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        let src_type = &self.source.1;
        let dst_type = &self.target.1;
        Cow::from(format!(
            "Types '{src_type}' and '{dst_type}' have different sizes."
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.source.0.clone(), self.target.0.clone()]
    }
}
