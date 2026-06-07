use crate::ast::CeriumType;
use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::qualifier::Qualifier;
use crate::error::{CompilerResult, CouldNotResolveVariable};
use crate::ranged::Ranged;
use chasm_ir::{Operand, inst};

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: Ranged<Qualifier>,
}

impl Compilable for Variable {
    fn compile(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        let op = Operand::Variable(self.name.1.to_string());
        let r#type = ctx
            .lookup(&self.name.1)
            .ok_or(CouldNotResolveVariable {
                name: self.name.clone(),
            })?
            .clone();
        then(&op, &r#type, ctx)
    }

    fn compile_mut(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        self.compile(ctx, &mut |op, r#type, ctx| {
            ctx.scope(|ctx| {
                let uuid = ctx.uuid();
                let var = ctx.push_var(uuid, r#type.clone());
                ctx.push_inst(inst!(Mov, op var.clone(), op op.clone()));
                then(&var, r#type, ctx)
            })
        })
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        Ok(())
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        let op = Operand::Variable(self.name.1.to_string());
        let r#type = ctx
            .lookup(&self.name.1)
            .ok_or(CouldNotResolveVariable {
                name: self.name.clone(),
            })?
            .clone();
        ctx.push_inst(inst!(Mov, op operand.clone(), op op.clone()));
        Ok(r#type)
    }
}
