use crate::ast::compilation::context::Context;
use crate::ast::compilation::{Compilable, ConstCompilable, ConstContext};
use crate::ast::expression::optimize::OptimizeExpression;
use crate::ast::{CeriumType, Expression};
use crate::error::CompilerResult;
use crate::ranged::Ranged;
use chasm_ir::{Operand, inst};

#[derive(Debug, Clone, PartialEq)]
pub struct Sizeof {
    pub r#type: Ranged<CeriumType>,
}

impl Compilable for Sizeof {
    fn compile(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        let size = self.r#type.1.size(ctx.structs())? as u16;
        then(&Operand::Constant(size), &CeriumType::U16, ctx)
    }

    fn compile_mut(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        let size = self.r#type.1.size(ctx.structs())? as u16;
        ctx.scope(|ctx| {
            let uuid = ctx.uuid();
            let var = ctx.push_var(uuid, CeriumType::U16);
            ctx.push_inst(inst!(Mov, op var.clone(), val size));
            then(&var, &CeriumType::U16, ctx)
        })
    }

    fn compile_unit(&self, _ctx: &mut Context) -> CompilerResult<()> {
        Ok(())
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        let size = self.r#type.1.size(ctx.structs())? as u16;
        ctx.push_inst(inst!(Mov, op operand.clone(), val size));
        Ok(CeriumType::U16)
    }
}

impl ConstCompilable for Sizeof {
    fn compile_const(&self, ctx: &mut ConstContext) -> CompilerResult<(Operand, CeriumType)> {
        let size = ctx.sizeof(&self.r#type.1)?;
        Ok((Operand::Constant(size as u16), CeriumType::U16))
    }
}

impl OptimizeExpression for Sizeof {
    fn optimize(self) -> Expression {
        Expression::Sizeof(Box::new(self))
    }
}
