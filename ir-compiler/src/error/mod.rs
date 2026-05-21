mod unexpected_character;
mod unexpected_eof;
mod unexpected_token;

pub use unexpected_character::UnexpectedCharacterError;
pub use unexpected_eof::UnexpectedEof;
pub use unexpected_token::UnexpectedTokenError;

pub type CompilerResult<T> = Result<T, CompilerError>;

#[derive(Debug, Clone, PartialEq)]
pub enum CompilerError {
    UnexpectedCharacterError(UnexpectedCharacterError),
    UnexpectedTokenError(UnexpectedTokenError),
    UnexpectedEof(UnexpectedEof),
}
