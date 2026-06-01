mod could_not_resolve_variable;
mod false_return_type;
mod incompatible_types;
mod type_alias_has_different_size;
mod unexpected_character;
mod unexpected_eof;
mod unexpected_token;
mod unparseable_constant;
mod unprocessable_unit;
mod value_not_dereferenceable;

use colored::{Color, Colorize};
pub use could_not_resolve_variable::CouldNotResolveVariable;
pub use false_return_type::FalseReturnType;
pub use incompatible_types::IncompatibleTypes;
use std::ops::{Add, Range, RangeInclusive};
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
    CouldNotResolveVariable(CouldNotResolveVariable),
}

pub trait FormatError {
    fn format(&self, src: &str) -> String;
}

impl FormatError for CompilerError {
    fn format(&self, src: &str) -> String {
        match self {
            CompilerError::UnexpectedCharacterError(_) => todo!(),
            CompilerError::UnexpectedTokenError(_) => todo!(),
            CompilerError::UnexpectedEof(_) => todo!(),
            CompilerError::UnparseableConstant(_) => todo!(),
            CompilerError::IncompatibleTypes(_) => todo!(),
            CompilerError::UnprocessableUnit(_) => todo!(),
            CompilerError::FalseReturnType(_) => todo!(),
            CompilerError::ValueNotDereferenceable(_) => todo!(),
            CompilerError::TypeAliasHasDifferentSize(_) => todo!(),
            CompilerError::CouldNotResolveVariable(_) => todo!(),
        }
    }
}

fn lines_within_range(
    code: &str,
    range: RangeInclusive<usize>,
) -> Vec<(usize, &str, Range<usize>)> {
    let mut offset = 0;
    let mut result = Vec::new();
    for (line_number, line) in code.split_inclusive('\n').enumerate() {
        let length = line.len();
        let line = line.trim_end();

        let start = range.start().saturating_sub(offset);
        let end = range.end().saturating_sub(offset + 1).min(line.len());

        if start < end {
            result.push((line_number, line, start..end));
        }

        offset += length;
    }
    result
}

fn highlight_lines(lines: &[(usize, &str, Range<usize>)]) -> String {
    lines
        .into_iter()
        .map(|(line_num, line, Range { start, end })| {
            format!(
                "{0:0>5} {3} {line}\n      {3} {1}{2}\n",
                line_num.add(1).to_string().color(Color::BrightBlue),
                " ".repeat(*start),
                "^".repeat(end - start).color(Color::Red),
                "|".color(Color::BrightBlue),
            )
        })
        .collect()
}
