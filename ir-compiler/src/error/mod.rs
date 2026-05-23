mod incompatible_types;
mod unexpected_character;
mod unexpected_eof;
mod unexpected_token;
mod unparseable_constant;
mod unprocessable_unit;

pub use incompatible_types::IncompatibleTypes;
pub use unexpected_character::UnexpectedCharacterError;
pub use unexpected_eof::UnexpectedEof;
pub use unexpected_token::UnexpectedTokenError;
pub use unparseable_constant::UnparseableConstant;
pub use unprocessable_unit::UnprocessableUnit;

pub type CompilerResult<T> = Result<T, CompilerError>;

#[derive(Debug, Clone, PartialEq)]
pub enum CompilerError {
    UnexpectedCharacterError(UnexpectedCharacterError),
    UnexpectedTokenError(UnexpectedTokenError),
    UnexpectedEof(UnexpectedEof),
    UnparseableConstant(UnparseableConstant),
    IncompatibleTypes(IncompatibleTypes),
    UnprocessableUnit(UnprocessableUnit),
}
