use crate::ast::compilation::context::Context;
use crate::ast::compilation::Compilable;
use crate::ast::expression::Expression;
use crate::ast::CeriumType;
use crate::error::CompilerResult;
use crate::ranged::Ranged;
use crate::snippet;
use chasm_ir::{inst, Instruction, Operand};

#[derive(Debug, Clone, PartialEq)]
pub struct Loop {
    pub body: Ranged<Expression>,
}

// TODO: implement break logic using self.body.iter_rec()...
impl Compilable for Loop {
    fn compile(
        &self,
        ctx: &mut Context,
    ) -> CompilerResult<(Vec<Instruction>, Option<(Operand, CeriumType)>)> {
        self.compile_unit(ctx).map(|snippet| (snippet, None))
    }

    fn compile_mut(
        &self,
        ctx: &mut Context,
    ) -> CompilerResult<(Vec<Instruction>, Option<(Operand, CeriumType)>)> {
        self.compile_unit(ctx).map(|snippet| (snippet, None))
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<Vec<Instruction>> {
        let label = ctx.label();
        let body = self.body.1.compile_unit(ctx)?;
        Ok(snippet![
            Instruction::Sublabel(label.clone()),
            body,
            inst!(Jmp, label),
        ])
    }

    fn compile_into(
        &self,
        ctx: &mut Context,
        operand: Operand,
    ) -> CompilerResult<(Vec<Instruction>, Option<CeriumType>)> {
        self.compile_unit(ctx).map(|snippet| (snippet, None))
    }
}
