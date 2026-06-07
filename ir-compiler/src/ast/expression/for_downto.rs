use crate::ast::CeriumType;
use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::expression::Expression;
use crate::ast::qualifier::Qualifier;
use crate::error::{CompilerResult, CouldNotResolveVariable, InvalidCounterType};
use crate::ranged::Ranged;
use crate::unprocessable_unit;
use chasm_ir::{Instruction, Operand, inst};

#[derive(Debug, Clone, PartialEq)]
pub struct ForDownTo {
    pub counter: Ranged<Qualifier>,
    pub limit: Ranged<Expression>,
    pub body: Ranged<Expression>,
}

impl Compilable for ForDownTo {
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

    /*
        <calc limit>
        jmp .cond
    .loop:
        add counter limit
        <body>
    .cond:
        sub counter limit
        jrnzdec counter .loop
     */
    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        // self.counter.1.compile_mut
        match ctx.lookup(&self.counter.1) {
            None => Err(CouldNotResolveVariable {
                name: self.counter.clone(),
            })?,
            Some(CeriumType::I16 | CeriumType::U16 | CeriumType::Reference(_)) => {}
            Some(encountered) => Err(InvalidCounterType {
                range: self.counter.0.clone(),
                encountered: encountered.clone(),
            })?,
        }

        self.limit.1.compile(ctx, &mut |op, r#type, ctx| {
            let dot_cond = ctx.label();
            let dot_loop = ctx.label();
            ctx.push_inst(inst!(Jmp, &dot_cond));
            ctx.push_inst(Instruction::Sublabel(dot_loop.clone()));
            ctx.push_inst(inst!(Add, self.counter.1, op op.clone()));
            self.body.1.compile_unit(ctx)?;
            ctx.push_inst(Instruction::Sublabel(dot_cond));
            ctx.push_inst(inst!(Sub, self.counter.1, op op.clone()));
            ctx.push_inst(inst!(Jrnzdec, self.counter.1, dot_loop));
            Ok(())
        })
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        unprocessable_unit!()
    }
}
