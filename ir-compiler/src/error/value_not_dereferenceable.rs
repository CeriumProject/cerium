use std::ops::RangeInclusive;
use crate::ast::CeriumType;
use crate::error::CompilerError;

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