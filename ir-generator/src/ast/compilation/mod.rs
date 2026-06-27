mod const_compilable;
pub mod context;

use crate::ast::CeriumType;
use crate::ast::compilation::context::Context;
use crate::error::CompilerResult;
use chasm_ir::Operand;

use crate::ranged::Ranged;
pub use const_compilable::{ConstCompilable, ConstContext};

pub trait Compilable {
    fn compile(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()>;

    fn compile_mut(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()>;

    // TODO: return type regardless to still allow for type checking (e.g. {a+b;} )
    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()>;

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType>;
}

impl<T: Compilable> Compilable for Ranged<T> {
    fn compile(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        self.1.compile(ctx, then)
    }

    fn compile_mut(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        self.1.compile_mut(ctx, then)
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        self.1.compile_unit(ctx)
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        self.1.compile_into(ctx, operand)
    }
}
