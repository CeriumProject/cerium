use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::{CeriumType, Expression};
use crate::error::{CompilerResult, ValueNotDereferenceable};
use crate::ranged::Ranged;
use chasm_ir::{Operand, inst};
use std::mem::MaybeUninit;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq)]
pub struct Dereference {
    pub inner: Ranged<Expression>,
}

impl Compilable for Dereference {
    fn compile(
        &self,
        ctx: &mut Context,
        then: impl FnOnce(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        self.compile_mut(ctx, then)
    }

    fn compile_mut(
        &self,
        ctx: &mut Context,
        then: impl FnOnce(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        self.inner.1.compile(ctx, |src, ty, ctx| {
            let uuid = ctx.uuid();
            let inner_type = deref_type(self.inner.0.clone(), ty)?;
            let dst = ctx.push_var(uuid, inner_type.clone());
            ctx.scope(|ctx| {
                ctx.push_inst(inst!(Read, op dst.clone(), op src.clone()));
                then(&dst, &inner_type, ctx)
            })
        })
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        self.inner.1.compile_unit(ctx)
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        let mut result = MaybeUninit::<CeriumType>::uninit();
        self.inner.1.compile(ctx, |src, ty, ctx| {
            result = MaybeUninit::new(deref_type(self.inner.0.clone(), ty)?);
            ctx.push_inst(inst!(Read, op operand.clone(), op src.clone()));
            Ok(())
        })?;
        Ok(unsafe { result.assume_init() })
    }
}

fn deref_type(range: RangeInclusive<usize>, ty: &CeriumType) -> CompilerResult<CeriumType> {
    match ty {
        CeriumType::Reference(inner) => Ok((**inner).clone()),
        _ => Err(ValueNotDereferenceable {
            range,
            r#type: ty.clone(),
        })?,
    }
}
