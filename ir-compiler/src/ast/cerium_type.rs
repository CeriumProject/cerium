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
