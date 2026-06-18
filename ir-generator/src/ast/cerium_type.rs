use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum CeriumType {
    // /// used for implicit types; will cause error if not resolved in ast optimization step
    // Unknown,
    I16,
    U16,
    F16,
    Reference(Box<CeriumType>),
    Function(Vec<CeriumType>, Option<Box<CeriumType>>),
}

impl Display for CeriumType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CeriumType::I16 => write!(f, "i16"),
            CeriumType::U16 => write!(f, "u16"),
            CeriumType::F16 => write!(f, "f16"),
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
        }
    }
}

impl CeriumType {
    pub fn size(&self) -> usize {
        1
    }

    // TODO: ::is_subtype_of
}
