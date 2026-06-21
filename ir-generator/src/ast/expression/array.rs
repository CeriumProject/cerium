use crate::ast::compilation::context::Context;
use crate::ast::compilation::{Compilable, ConstCompilable, ConstContext};
use crate::ast::expression::optimize::{OptimizeExpression, OptimizeRangedExpression};
use crate::ast::{CeriumType, Expression};
use crate::error::CompilerResult;
use crate::ranged::Ranged;
use chasm_ir::Operand;

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    pub elements: Vec<Ranged<Expression>>,
}

impl Compilable for Array {
    fn compile(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        todo!("can only be const")
    }

    fn compile_mut(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        todo!("can only be const")
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        todo!("can only be const")
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        todo!("can only be const")
    }
}

impl ConstCompilable for Array {
    fn compile_const(&self, ctx: &mut ConstContext) -> CompilerResult<(Operand, CeriumType)> {
        todo!("can only be ref")
    }
}

impl OptimizeExpression for Array {
    fn optimize(self) -> Expression {
        let elements = self
            .elements
            .into_iter()
            .map(Ranged::<Expression>::optimize)
            .collect();
        Expression::Array(Box::new(Array { elements }))
    }
}
