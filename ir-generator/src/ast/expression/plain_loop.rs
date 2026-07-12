use crate::ast::CeriumType;
use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::expression::Expression;
use crate::ast::expression::optimize::{OptimizeExpression, OptimizeRangedExpression};
use crate::error::{CompilerResult, UnprocessableUnit};
use crate::ranged::Ranged;
use chasm_ir::{Instruction, Operand, inst};

#[derive(Debug, Clone, PartialEq)]
pub struct Loop {
    pub body: Ranged<Expression>,
}

// TODO: implement break logic using self.body.iter_rec()...
impl Compilable for Loop {
    fn compile(
        &self,
        _ctx: &mut Context,
        _then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        Err(UnprocessableUnit {
            range: self.body.0.clone(),
        })?
    }

    fn compile_mut(
        &self,
        _ctx: &mut Context,
        _then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        Err(UnprocessableUnit {
            range: self.body.0.clone(),
        })?
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        ctx.scope(|ctx| {
            let label = ctx.label();
            ctx.push_inst(Instruction::Sublabel(label.clone()));
            self.body.1.compile_unit(ctx)?;
            ctx.push_inst(inst!(Jmp, format!(".{label}")));
            Ok(())
        })
    }

    fn compile_into(&self, _ctx: &mut Context, _operand: &Operand) -> CompilerResult<CeriumType> {
        Err(UnprocessableUnit {
            range: self.body.0.clone(),
        })?
    }
}

impl OptimizeExpression for Loop {
    fn optimize(self) -> Expression {
        Expression::Loop(Box::new(Loop {
            body: self.body.optimize(),
        }))
    }
}
