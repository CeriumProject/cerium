use crate::ast::CeriumType;
use crate::error::CompilerError;
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
