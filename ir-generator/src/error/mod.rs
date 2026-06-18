mod cannot_cast_type;
mod could_not_resolve_variable;
mod false_return_type;
mod incompatible_types;
mod index_must_be_integer;
mod invalid_counter_type;
mod invalid_parameter_amount;
mod mismatched_assignment_type;
mod mismatched_parameter_type;
mod type_alias_has_different_size;
mod unexpected_character;
mod unexpected_eof;
mod unexpected_token;
mod unparseable_constant;
mod unprocessable_unit;
mod value_not_dereferenceable;
mod value_not_invocable;
mod value_not_referenceable;

pub use cannot_cast_type::CannotCastType;
use colored::{Color, Colorize};
pub use could_not_resolve_variable::CouldNotResolveVariable;
pub use false_return_type::FalseReturnType;
pub use incompatible_types::IncompatibleTypes;
pub use index_must_be_integer::IndexMustBeInteger;
pub use invalid_counter_type::InvalidCounterType;
pub use invalid_parameter_amount::InvalidParameterAmount;
pub use mismatched_assignment_type::MismatchedAssignmentType;
pub use mismatched_parameter_type::MismatchedParameterType;
use std::borrow::Cow;
use std::ops::{Add, RangeInclusive};
pub use type_alias_has_different_size::TypeAliasHasDifferentSize;
pub use unexpected_character::UnexpectedCharacterError;
pub use unexpected_eof::UnexpectedEof;
pub use unexpected_token::UnexpectedTokenError;
pub use unparseable_constant::UnparseableConstant;
pub use unprocessable_unit::UnprocessableUnit;
pub use value_not_dereferenceable::ValueNotDereferenceable;
pub use value_not_invocable::ValueNotInvocable;
pub use value_not_referenceable::ValueNotReferenceable;

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
    CouldNotResolveVariable(CouldNotResolveVariable),
    ValueNotInvocable(ValueNotInvocable),
    InvalidParameterAmount(InvalidParameterAmount),
    MismatchedParameterType(MismatchedParameterType),
    CannotCastType(CannotCastType),
    MismatchedAssignmentType(MismatchedAssignmentType),
    InvalidCounterType(InvalidCounterType),
    ValueNotReferenceable(ValueNotReferenceable),
    IndexMustBeInteger(IndexMustBeInteger),
}

pub trait FormatError {
    fn error_message(&self) -> Cow<str>;
    fn error_explanation(&self) -> Cow<str>;
    fn highlights(&self) -> Vec<RangeInclusive<usize>>;
    fn format(&self, src: &str) -> String {
        let highlights = self.highlights().get(0).unwrap().clone(); // TODO: highlight ALL
        let lines = lines_within_range(src, highlights);
        let underlined = highlight_lines(&lines);
        let message = self.error_message().color(Color::Red);
        let explanation = format!(": {}", self.error_explanation()).color(Color::BrightWhite);
        format!("{message}{explanation}\n{underlined}")
    }
}

impl CompilerError {
    fn as_formattable(&self) -> &dyn FormatError {
        match self {
            CompilerError::UnexpectedCharacterError(error) => error,
            CompilerError::UnexpectedTokenError(error) => error,
            CompilerError::UnexpectedEof(error) => error,
            CompilerError::UnparseableConstant(error) => error,
            CompilerError::IncompatibleTypes(error) => error,
            CompilerError::UnprocessableUnit(error) => todo!(),
            CompilerError::FalseReturnType(error) => error,
            CompilerError::ValueNotDereferenceable(error) => error,
            CompilerError::TypeAliasHasDifferentSize(error) => error,
            CompilerError::CouldNotResolveVariable(error) => error,
            CompilerError::ValueNotInvocable(error) => error,
            CompilerError::InvalidParameterAmount(error) => error,
            CompilerError::MismatchedParameterType(error) => error,
            CompilerError::CannotCastType(error) => error,
            CompilerError::MismatchedAssignmentType(error) => error,
            CompilerError::InvalidCounterType(error) => error,
            CompilerError::ValueNotReferenceable(error) => error,
            CompilerError::IndexMustBeInteger(error) => error,
        }
    }
}

impl FormatError for CompilerError {
    fn error_message(&self) -> Cow<str> {
        self.as_formattable().error_message()
    }

    fn error_explanation(&self) -> Cow<str> {
        self.as_formattable().error_explanation()
    }

    fn highlights(&self) -> Vec<RangeInclusive<usize>> {
        self.as_formattable().highlights()
    }

    fn format(&self, src: &str) -> String {
        self.as_formattable().format(src)
    }
}

fn lines_within_range(
    code: &str,
    range: RangeInclusive<usize>,
) -> Vec<(usize, &str, RangeInclusive<usize>)> {
    let mut offset = 0;
    let mut result = Vec::new();
    for (line_number, line) in code.split_inclusive('\n').enumerate() {
        let length = line.len();
        let line = line.trim_end();

        let start = range.start().saturating_sub(offset);
        let end = range.end().add(1).saturating_sub(offset).min(line.len());

        if start < end {
            result.push((line_number, line, start..=end - 1));
        }

        offset += length;
    }
    result
}

fn highlight_lines(lines: &[(usize, &str, RangeInclusive<usize>)]) -> String {
    lines
        .into_iter()
        .map(|(line_num, line, range)| {
            format!(
                "{0:0>5} {3} {line}\n      {3} {1}{2}\n",
                line_num.add(1).to_string().color(Color::BrightBlue),
                " ".repeat(*range.start()),
                "^".repeat(*range.end() + 1 - *range.start())
                    .color(Color::Red),
                "|".color(Color::BrightBlue),
            )
        })
        .collect()
}
