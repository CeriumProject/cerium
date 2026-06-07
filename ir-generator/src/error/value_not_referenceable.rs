use crate::error::CompilerError;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, PartialEq)]
pub struct ValueNotReferenceable {
    pub range: RangeInclusive<usize>,
}

impl From<ValueNotReferenceable> for CompilerError {
    fn from(value: ValueNotReferenceable) -> Self {
        CompilerError::ValueNotReferenceable(value)
    }
}
