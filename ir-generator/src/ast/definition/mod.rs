mod constant;
mod function;
mod structure;

pub use crate::ast::definition::constant::Constant;
pub use crate::ast::definition::function::Function;
pub use crate::ast::definition::structure::Structure;
use crate::ast::{CeriumType, Qualifier};
use crate::error::CompilerResult;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Definition {
    Function(Function),
    Constant(Constant),
    Structure(Structure),
}

impl Definition {
    pub fn as_global(&self) -> Option<(Qualifier, CeriumType)> {
        match self {
            Definition::Function(function) => Some((function.name.1.clone(), function.signature())),
            Definition::Constant(constant) => {
                Some((constant.name.1.clone(), constant.r#type.1.clone()))
            }
            Definition::Structure(structure) => None,
        }
    }

    pub fn as_struct(&self) -> Option<(Qualifier, Vec<(Qualifier, CeriumType)>)> {
        match self {
            Definition::Structure(structure) => Some(structure.signature()),
            _ => None,
        }
    }

    pub fn compile(
        &self,
        globals: &HashMap<Qualifier, CeriumType>,
        structs: &HashMap<Qualifier, Vec<(Qualifier, CeriumType)>>,
    ) -> CompilerResult<Vec<chasm_ir::Section>> {
        match self {
            Definition::Function(function) => function.compile(globals, structs),
            Definition::Constant(constant) => constant.compile(globals, structs),
            Definition::Structure(_) => Ok(Vec::new()),
        }
    }

    pub fn optimize(self) -> Self {
        match self {
            Definition::Function(function) => Definition::Function(function.optimize()),
            Definition::Constant(constant) => Definition::Constant(constant.optimize()),
            definition => definition,
        }
    }
}
