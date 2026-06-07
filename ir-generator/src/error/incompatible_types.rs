use crate::ast::CeriumType;
use crate::error::CompilerError;
use crate::ranged::Ranged;

#[derive(Clone, Debug, PartialEq)]
pub struct IncompatibleTypes {
    pub lhs: Ranged<CeriumType>,
    pub rhs: Ranged<CeriumType>,
}

impl From<IncompatibleTypes> for CompilerError {
    fn from(e: IncompatibleTypes) -> Self {
        CompilerError::IncompatibleTypes(e)
    }
}
