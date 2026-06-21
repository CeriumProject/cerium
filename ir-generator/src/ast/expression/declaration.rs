use crate::ast::CeriumType;
use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::expression::Expression;
use crate::ast::expression::optimize::{OptimizeExpression, OptimizeRangedExpression};
use crate::ast::qualifier::Qualifier;
use crate::error::CompilerResult;
use crate::ranged::Ranged;
use crate::unprocessable_unit;
use chasm_ir::Operand;

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub name: Ranged<Qualifier>,
    // pub r#type: Ranged<CeriumType>,
    pub value: Ranged<Expression>,
}

impl Compilable for Declaration {
    fn compile(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        unprocessable_unit!()
    }

    fn compile_mut(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        unprocessable_unit!()
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        // placeholder
        let op = ctx.push_var(self.name.1.clone(), CeriumType::I16);

        let r#type = self.value.1.compile_into(ctx, &op)?;
        ctx.change_type(&self.name.1, r#type);
        Ok(())
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        unprocessable_unit!()
    }
}

impl OptimizeExpression for Declaration {
    fn optimize(self) -> Expression {
        Expression::Declaration(Box::new(Declaration {
            name: self.name,
            value: self.value.optimize(),
        }))
    }
}
