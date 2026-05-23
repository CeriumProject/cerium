mod function;

use crate::ast::compilation::context::Context;
pub use crate::ast::definition::function::Function;
use crate::ast::{CeriumType, Qualifier};
use crate::error::CompilerResult;

#[derive(Debug, Clone, PartialEq)]
pub enum Definition {
    Function(Function),
}

impl Definition {
    pub fn as_global(&self) -> Option<(Qualifier, CeriumType)> {
        match self {
            Definition::Function(function) => Some((function.name.1.clone(), function.signature())),
        }
    }

    pub fn compile(&self, ctx: Context) -> Option<CompilerResult<chasm_ir::Section>> {
        match self {
            Definition::Function(function) => Some(function.compile(ctx)),
        }
    }
}
