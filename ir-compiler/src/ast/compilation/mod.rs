pub mod context;
pub mod snippet;
pub mod amend;

use crate::ast::compilation::context::Context;
use crate::ast::CeriumType;
use crate::error::CompilerResult;

// TODO: add default implementations based on compile_mut
// TODO: add context
pub trait Compilable {
    fn compile(
        &self,
        ctx: &mut Context,
    ) -> CompilerResult<(
        Vec<chasm_ir::Instruction>,
        Option<(chasm_ir::Operand, CeriumType)>,
    )>;

    fn compile_mut(
        &self,
        ctx: &mut Context,
    ) -> CompilerResult<(
        Vec<chasm_ir::Instruction>,
        Option<(chasm_ir::Operand, CeriumType)>,
    )>;

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<Vec<chasm_ir::Instruction>>;

    fn compile_into(
        &self,
        ctx: &mut Context,
        operand: chasm_ir::Operand,
    ) -> CompilerResult<(Vec<chasm_ir::Instruction>, Option<CeriumType>)>;
}
