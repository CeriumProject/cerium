use crate::error::CompilerError;

#[derive(Clone, Debug, PartialEq)]
pub struct UnexpectedEof;

impl Into<CompilerError> for UnexpectedEof {
    fn into(self) -> CompilerError {
        CompilerError::UnexpectedEof(self)
    }
}
