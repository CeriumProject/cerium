use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::{CeriumType, Expression};
use crate::error::{CompilerResult, ValueNotReferenceable};
use crate::ranged::Ranged;
use chasm_ir::{Instruction, Operand};
use std::mem::MaybeUninit;

#[derive(Debug, Clone, PartialEq)]
pub struct Reference {
    pub inner: Ranged<Expression>,
}

impl Compilable for Reference {
    fn compile(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        self.compile_mut(ctx, then)
    }

    fn compile_mut(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        self.inner.1.compile(ctx, &mut |op, r#type, ctx| {
            let Operand::Variable(variable) = op else {
                Err(ValueNotReferenceable {
                    range: self.inner.0.clone(),
                })?
            };
            let result_type = CeriumType::Reference(Box::new(r#type.clone()));
            let uuid = ctx.uuid();
            let new_op = ctx.push_var(uuid.clone(), result_type.clone());
            ctx.push_inst(Instruction::Reference(new_op.clone(), variable.clone()));
            then(&new_op, &result_type, ctx)
        })
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        self.inner.1.compile_unit(ctx)
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        let mut result_type = MaybeUninit::uninit();
        self.inner.1.compile(ctx, &mut |op, r#type, ctx| {
            let Operand::Variable(variable) = op else {
                Err(ValueNotReferenceable {
                    range: self.inner.0.clone(),
                })?
            };
            ctx.push_inst(Instruction::Reference(operand.clone(), variable.clone()));
            result_type = MaybeUninit::new(r#type.clone());
            Ok(())
        })?;
        Ok(CeriumType::Reference(Box::new(unsafe {
            result_type.assume_init()
        })))
    }
}
