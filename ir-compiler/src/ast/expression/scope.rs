use crate::ast::CeriumType;
use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::expression::Expression;
use crate::error::{CompilerResult, UnprocessableUnit};
use crate::ranged::Ranged;
use chasm_ir::Operand;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    pub statements: Vec<Ranged<Expression>>,
    pub result: Option<Ranged<Expression>>,
}

impl Compilable for Scope {
    fn compile(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        ctx.scope(|ctx| {
            for (_, statement) in &self.statements {
                statement.compile_unit(ctx)?;
            }
            self.result
                .as_ref()
                .ok_or(UnprocessableUnit { range: todo!() })?
                .1
                .compile(ctx, then)
        })
    }

    fn compile_mut(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        ctx.scope(|ctx| {
            for (_, statement) in &self.statements {
                statement.compile_unit(ctx)?;
            }
            self.result
                .as_ref()
                .ok_or(UnprocessableUnit { range: todo!() })?
                .1
                .compile_mut(ctx, then)
        })
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        ctx.scope(|ctx| {
            for (_, statement) in &self.statements {
                statement.compile_unit(ctx)?;
            }
            if let Some((_, result)) = &self.result {
                result.compile_unit(ctx)?;
            }
            Ok(())
        })
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        ctx.scope(|ctx| {
            for (_, statement) in &self.statements {
                statement.compile_unit(ctx)?;
            }
            self.result
                .as_ref()
                .ok_or(UnprocessableUnit { range: todo!() })?
                .1
                .compile_into(ctx, operand)
        })
    }
}
