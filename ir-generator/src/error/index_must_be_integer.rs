use crate::ast::CeriumType;
use crate::error::{CompilerError, FormatError};
use std::borrow::Cow;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct IndexMustBeInteger {
    pub range: RangeInclusive<usize>,
    pub encountered: CeriumType,
}

impl From<IndexMustBeInteger> for CompilerError {
    fn from(e: IndexMustBeInteger) -> Self {
        CompilerError::IndexMustBeInteger(e)
    }
}

impl FormatError for IndexMustBeInteger {
    fn error_message(&self) -> Cow<str> {
        Cow::from("Indexation Error")
    }

    fn error_explanation(&self) -> Cow<str> {
        let index_type = &self.encountered;
        Cow::from(format!(
            "Index must be of integer type, not '{index_type}'."
        ))
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        vec![self.range.clone()]
    }
}
