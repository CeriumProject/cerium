mod constant;
mod function;

pub use crate::ast::definition::constant::Constant;
pub use crate::ast::definition::function::Function;
use crate::ast::{CeriumType, Qualifier};
use crate::error::CompilerResult;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Definition {
    Function(Function),
    Constant(Constant),
}

impl Definition {
    pub fn as_global(&self) -> Option<(Qualifier, CeriumType)> {
        match self {
            Definition::Function(function) => Some((function.name.1.clone(), function.signature())),
            Definition::Constant(constant) => {
                Some((constant.name.1.clone(), constant.r#type.1.clone()))
            }
        }
    }

    pub fn compile(
        &self,
        globals: &HashMap<Qualifier, CeriumType>,
    ) -> CompilerResult<Vec<chasm_ir::Section>> {
        match self {
            Definition::Function(function) => function.compile(globals),
            Definition::Constant(constant) => constant.compile(globals),
        }
    }
}
