use crate::error::CompilerError;

#[derive(Clone, Debug, PartialEq)]
pub struct UnexpectedCharacterError {
    pub character: char,
    pub idx: usize,
}

impl Into<CompilerError> for UnexpectedCharacterError {
    fn into(self) -> CompilerError {
        CompilerError::UnexpectedCharacterError(self)
    }
}
