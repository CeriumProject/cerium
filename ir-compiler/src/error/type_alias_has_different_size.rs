use crate::ast::CeriumType;
use crate::error::CompilerError;
use crate::ranged::Ranged;

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
