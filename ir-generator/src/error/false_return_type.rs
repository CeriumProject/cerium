use crate::ast::{CeriumType, Qualifier};
use crate::error::CompilerError;
use crate::ranged::Ranged;

#[derive(Clone, Debug, PartialEq)]
pub struct FalseReturnType {
    pub function: Ranged<Qualifier>,
    pub expected: Option<Ranged<CeriumType>>,
    pub actual: Option<Ranged<CeriumType>>,
}

impl From<FalseReturnType> for CompilerError {
    fn from(err: FalseReturnType) -> Self {
        CompilerError::FalseReturnType(err)
    }
}
