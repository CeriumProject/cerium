use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
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
        _ctx: &mut Context,
        _then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        todo!()
    }

    fn compile_mut(
        &self,
        _ctx: &mut Context,
        _then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        todo!()
    }

    fn compile_unit(&self, _ctx: &mut Context) -> CompilerResult<()> {
        todo!()
    }

    fn compile_into(&self, _ctx: &mut Context, _operand: &Operand) -> CompilerResult<CeriumType> {
        todo!()
    }
}

impl OptimizeExpression for StringConstant {
    fn optimize(self) -> Expression {
        Expression::StringConstant(Box::new(self))
    }
}
