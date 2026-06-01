mod false_return_type;
mod incompatible_types;
mod type_alias_has_different_size;
mod unexpected_character;
mod unexpected_eof;
mod unexpected_token;
mod unparseable_constant;
mod unprocessable_unit;
mod value_not_dereferenceable;

pub use false_return_type::FalseReturnType;
pub use incompatible_types::IncompatibleTypes;
pub use type_alias_has_different_size::TypeAliasHasDifferentSize;
pub use unexpected_character::UnexpectedCharacterError;
pub use unexpected_eof::UnexpectedEof;
pub use unexpected_token::UnexpectedTokenError;
pub use unparseable_constant::UnparseableConstant;
pub use unprocessable_unit::UnprocessableUnit;
pub use value_not_dereferenceable::ValueNotDereferenceable;

pub type CompilerResult<T> = Result<T, CompilerError>;

#[derive(Debug, Clone, PartialEq)]
pub enum CompilerError {
    UnexpectedCharacterError(UnexpectedCharacterError),
    UnexpectedTokenError(UnexpectedTokenError),
    UnexpectedEof(UnexpectedEof),
    UnparseableConstant(UnparseableConstant),
    IncompatibleTypes(IncompatibleTypes),
    UnprocessableUnit(UnprocessableUnit),
    FalseReturnType(FalseReturnType),
    ValueNotDereferenceable(ValueNotDereferenceable),
    TypeAliasHasDifferentSize(TypeAliasHasDifferentSize),
}
