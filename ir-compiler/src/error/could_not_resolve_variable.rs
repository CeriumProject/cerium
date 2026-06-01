use crate::ast::Qualifier;
use crate::error::CompilerError;
use crate::ranged::Ranged;

#[derive(Clone, Debug, PartialEq)]
pub struct CouldNotResolveVariable {
    pub(crate) name: Ranged<Qualifier>,
}

impl From<CouldNotResolveVariable> for CompilerError {
    fn from(value: CouldNotResolveVariable) -> Self {
        CompilerError::CouldNotResolveVariable(value)
    }
}
