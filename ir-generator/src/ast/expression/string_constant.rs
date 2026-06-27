use crate::ast::compilation::context::Context;
use crate::ast::compilation::{Compilable, ConstCompilable};
use crate::ast::optimize::OptimizeExpression;
use crate::ast::{CeriumType, Expression};
use crate::error::CompilerResult;
use crate::ranged::Ranged;
use chasm_ir::Operand;

#[derive(Debug, Clone, PartialEq)]
pub struct StringConstant {
    pub value: Ranged<String>,
}

impl Compilable for StringConstant {
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

impl OptimizeExpression for StringConstant {
    fn optimize(self) -> Expression {
        Expression::StringConstant(Box::new(self))
    }
}
