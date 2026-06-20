use crate::ast::compilation::context::Context;
use crate::ast::compilation::{Compilable, ConstCompilable, ConstContext};
use crate::ast::{CeriumType, Expression, Qualifier};
use crate::error::{CompilerResult, CouldNotResolveType};
use crate::ranged::Ranged;
use chasm_ir::{Instruction, Operand};

#[derive(Debug, Clone, PartialEq)]
pub struct StructInitialization {
    pub name: Ranged<Qualifier>,
    pub fields: Vec<(Ranged<Qualifier>, Ranged<Expression>)>,
}

impl Compilable for StructInitialization {
    fn compile(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        todo!()
    }

    fn compile_mut(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        todo!()
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        todo!()
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        todo!()
    }
}
