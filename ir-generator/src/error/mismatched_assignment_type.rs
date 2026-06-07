use crate::ast::CeriumType;
use crate::error::CompilerError;
use crate::ranged::Ranged;

#[derive(Clone, Debug, PartialEq)]
pub struct MismatchedAssignmentType {
    pub destination: Ranged<CeriumType>,
    pub source: Ranged<CeriumType>,
}

impl From<MismatchedAssignmentType> for CompilerError {
    fn from(error: MismatchedAssignmentType) -> CompilerError {
        CompilerError::MismatchedAssignmentType(error)
    }
}
