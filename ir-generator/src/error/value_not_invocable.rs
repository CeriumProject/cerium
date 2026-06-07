use crate::ast::CeriumType;
use crate::error::CompilerError;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct ValueNotInvocable {
    pub range: RangeInclusive<usize>,
    pub r#type: CeriumType,
}

impl From<ValueNotInvocable> for CompilerError {
    fn from(value: ValueNotInvocable) -> Self {
        CompilerError::ValueNotInvocable(value)
    }
}
