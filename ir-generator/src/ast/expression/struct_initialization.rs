use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::expression::optimize::{OptimizeExpression, OptimizeRangedExpression};
use crate::ast::{CeriumType, Expression, Qualifier};
use crate::error::CompilerResult;
use crate::ranged::Ranged;
use chasm_ir::Operand;

#[derive(Debug, Clone, PartialEq)]
pub struct StructInitialization {
    pub name: Ranged<Qualifier>,
    pub fields: Vec<(Ranged<Qualifier>, Ranged<Expression>)>,
}

impl Compilable for StructInitialization {
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

impl OptimizeExpression for StructInitialization {
    fn optimize(self) -> Expression {
        Expression::StructInitialization(Box::new(StructInitialization {
            name: self.name,
            fields: self
                .fields
                .into_iter()
                .map(|(name, value)| (name, value.optimize()))
                .collect(),
        }))
    }
}
