pub mod amend;
pub mod context;
pub mod snippet;

use crate::ast::CeriumType;
use crate::ast::compilation::context::Context;
use crate::error::CompilerResult;
use chasm_ir::Operand;

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
