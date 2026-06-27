mod cannot_cast_type;
mod cannot_read_fields_on_type;
mod could_not_resolve_field;
mod could_not_resolve_type;
mod could_not_resolve_variable;
mod false_field_type;
mod false_return_type;
mod incompatible_bitwise_operation_types;
mod incompatible_types;
mod index_must_be_integer;
mod invalid_char_length;
mod invalid_counter_type;
mod invalid_parameter_amount;
mod mismatched_assignment_type;
mod mismatched_parameter_type;
mod type_alias_has_different_size;
mod unassigned_field;
mod unexpected_character;
mod unexpected_eof;
mod unexpected_token;
mod unparseable_number;
mod unprocessable_unit;
mod value_not_dereferenceable;
mod value_not_invocable;
mod value_not_referenceable;

pub use crate::error::cannot_read_fields_on_type::CannotReadFieldsOnType;
pub use crate::error::could_not_resolve_field::CouldNotResolveField;
pub use crate::error::false_field_type::FalseFieldType;
pub use crate::error::unassigned_field::UnassignedField;
pub use cannot_cast_type::CannotCastType;
use colored::{Color, Colorize};
pub use could_not_resolve_type::CouldNotResolveType;
pub use could_not_resolve_variable::CouldNotResolveVariable;
pub use false_return_type::FalseReturnType;
pub use incompatible_bitwise_operation_types::IncompatibleBitwiseOperationTypes;
pub use incompatible_types::IncompatibleTypes;
pub use index_must_be_integer::IndexMustBeInteger;
pub use invalid_char_length::InvalidCharLength;
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
pub use unparseable_number::UnparseableNumber;
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
    UnparseableNumber(UnparseableNumber),
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
    CouldNotResolveType(CouldNotResolveType),
    CouldNotResolveField(CouldNotResolveField),
    CannotReadFieldsOnType(CannotReadFieldsOnType),
    FalseFieldType(FalseFieldType),
    UnassignedField(UnassignedField),
    InvalidCharLength(InvalidCharLength),
    IncompatibleBitwiseOperationTypes(IncompatibleBitwiseOperationTypes),
}

pub trait FormatError {
    fn error_message(&self) -> Cow<str>;
    fn error_explanation(&self) -> Cow<str>;
    fn highlights(&self) -> Vec<RangeInclusive<usize>>;
    fn format(&self, src: &str) -> String {
        let highlights = self.highlights();
        let lines = lines_within_ranges(src, &highlights);
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
            CompilerError::UnparseableNumber(error) => error,
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
            CompilerError::CouldNotResolveType(error) => error,
            CompilerError::CouldNotResolveField(error) => error,
            CompilerError::CannotReadFieldsOnType(error) => error,
            CompilerError::FalseFieldType(error) => error,
            CompilerError::UnassignedField(error) => error,
            CompilerError::InvalidCharLength(error) => error,
            CompilerError::IncompatibleBitwiseOperationTypes(error) => error,
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

fn lines_within_ranges<'a, 'b>(
    code: &'a str,
    ranges: &'b [RangeInclusive<usize>],
) -> Vec<(usize, &'a str, Vec<RangeInclusive<usize>>)> {
    let mut offset = 0;
    let mut result = Vec::new();
    for (line_number, line) in code.split_inclusive('\n').enumerate() {
        let length = line.len();
        let line = line.trim_end();

        let mut sub_result = Vec::new();
        for range in ranges {
            let start = range.start().saturating_sub(offset);
            let end = range.end().add(1).saturating_sub(offset).min(line.len());

            if start < end {
                sub_result.push(start..=end - 1);
            }
        }
        if !sub_result.is_empty() {
            result.push((line_number, line, sub_result));
        }

        offset += length;
    }
    result
}

fn highlight_lines(lines: &[(usize, &str, Vec<RangeInclusive<usize>>)]) -> String {
    lines
        .into_iter()
        .map(|(line_num, line, ranges)| {
            let highlight_width = ranges
                .iter()
                .map(|range| *range.end())
                .max()
                .unwrap_or_default()
                + 1;
            let highlight = (0..highlight_width)
                .map(|idx| {
                    if ranges.iter().any(|range| range.contains(&idx)) {
                        '^'
                    } else {
                        ' '
                    }
                })
                .collect::<String>();
            format!(
                "{0:0>5} {2} {line}\n      {2} {1}\n",
                line_num.add(1).to_string().color(Color::BrightBlue),
                highlight.color(Color::Red),
                "|".color(Color::BrightBlue),
            )
        })
        .collect()
}
