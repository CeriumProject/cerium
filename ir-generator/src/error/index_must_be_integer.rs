use crate::ast::CeriumType;
use crate::error::CompilerError;
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
