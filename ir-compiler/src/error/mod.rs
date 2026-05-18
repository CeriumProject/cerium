use crate::error::unexpected_character::UnexpectedCharacterError;

pub mod unexpected_character;

pub type CompilerResult<T> = Result<T, CompilerError>;

#[derive(Debug, Clone, PartialEq)]
pub enum CompilerError {
    UnexpectedCharacterError(UnexpectedCharacterError),
}
