use crate::ast::Qualifier;
use crate::error::CompilerResult;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum CeriumType {
    // /// used for implicit types; will cause error if not resolved in ast optimization step
    // Unknown,
    I16,
    U16,
    F16,
    Bool,
    Char,
    Reference(Box<CeriumType>),
    Function(Vec<CeriumType>, Option<Box<CeriumType>>),
    Struct(Qualifier),
    /// is subtype of every other type
    /// allows implicit conversion for nullptr
    Undefined(usize),
    Any(usize),
}

impl Display for CeriumType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CeriumType::I16 => write!(f, "i16"),
            CeriumType::U16 => write!(f, "u16"),
            CeriumType::F16 => write!(f, "f16"),
            CeriumType::Bool => write!(f, "bool"),
            CeriumType::Char => write!(f, "char"),
            CeriumType::Reference(inner) => write!(f, "&{inner}"),
            CeriumType::Function(params, result) => {
                write!(
                    f,
                    "fn({0})",
                    params
                        .iter()
                        .map(CeriumType::to_string)
                        .collect::<Vec<_>>()
                        .join(",")
                )?;
                match result {
                    Some(result) => write!(f, " -> {result}"),
                    None => Ok(()),
                }
            }
            CeriumType::Struct(name) => write!(f, "{name}"),
            CeriumType::Undefined(size) => write!(f, "undefined[{size}]"),
            CeriumType::Any(size) => write!(f, "any[{size}]"),
        }
    }
}

// TODO: Result -> Option
impl CeriumType {
    pub fn size(
        &self,
        structs: &HashMap<Qualifier, Vec<(Qualifier, CeriumType)>>,
    ) -> CompilerResult<usize> {
        match self {
            CeriumType::I16
            | CeriumType::U16
            | CeriumType::F16
            | CeriumType::Bool
            | CeriumType::Char
            | CeriumType::Reference(_)
            | CeriumType::Function(_, _) => Ok(1),
            CeriumType::Struct(name) => match structs.get(name) {
                Some(fields) => fields
                    .iter()
                    .map(|(_, field_type)| field_type.size(structs))
                    .sum(),
                None => todo!(),
            },
            CeriumType::Undefined(size) | CeriumType::Any(size) => Ok(*size),
        }
    }

    pub fn is_subtype_of(
        &self,
        other: &CeriumType,
        structs: &HashMap<Qualifier, Vec<(Qualifier, CeriumType)>>,
    ) -> CompilerResult<bool> {
        match (self, other) {
            (CeriumType::Undefined(size), rhs) => Ok(*size == rhs.size(structs)?),
            (lhs, CeriumType::Any(size)) => Ok(lhs.size(structs)? == *size),
            (CeriumType::Reference(lhs), CeriumType::Reference(rhs)) => {
                lhs.is_subtype_of(rhs.as_ref(), structs)
            }
            (lhs, rhs) => Ok(*lhs == *rhs),
        }
    }
}
