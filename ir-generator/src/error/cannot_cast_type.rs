use crate::ast::CeriumType;
use crate::error::CompilerError;
use crate::ranged::Ranged;

#[derive(Clone, Debug, PartialEq)]
pub struct CannotCastType {
    pub from: Ranged<CeriumType>,
    pub to: Ranged<CeriumType>,
}

impl From<CannotCastType> for CompilerError {
    fn from(value: CannotCastType) -> CompilerError {
        CompilerError::CannotCastType(value)
    }
}
