use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::{CeriumType, Expression, Qualifier};
use crate::error::{CannotReadFieldsOnType, CompilerResult, CouldNotResolveField};
use crate::ranged::Ranged;
use chasm_ir::{Operand, inst};
use std::mem::MaybeUninit;

#[derive(Debug, Clone, PartialEq)]
pub struct FieldAccess {
    pub structure: Ranged<Expression>,
    pub field: Ranged<Qualifier>,
}

impl Compilable for FieldAccess {
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
        self.structure.1.compile(ctx, &mut |op, r#type, ctx| {
            let CeriumType::Reference(struct_type) = r#type else {
                Err(CannotReadFieldsOnType {
                    range: self.structure.0.clone(),
                    r#type: r#type.clone(),
                })?
            };
            let CeriumType::Struct(struct_name) = struct_type.as_ref() else {
                Err(CannotReadFieldsOnType {
                    range: self.structure.0.clone(),
                    r#type: r#type.clone(),
                })?
            };
            let (offset, field_type) = ctx
                .field_offset_and_type(struct_name, &self.field.1)
                .ok_or_else(|| CouldNotResolveField {
                    maybe_struct_type: struct_type.as_ref().clone(),
                    name: self.field.clone(),
                })?;
            ctx.scope(|ctx| {
                let uuid = ctx.uuid();
                let var = ctx.push_var(uuid, field_type.clone());
                ctx.push_inst(inst!(Mov, op var.clone(), op op.clone()));
                ctx.push_inst(inst!(Lookup, op var.clone(), val offset as u16));
                then(&var, &field_type, ctx)
            })
        })
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        self.structure.1.compile_unit(ctx)
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        let mut result = MaybeUninit::uninit();
        self.structure.1.compile(ctx, &mut |op, r#type, ctx| {
            let CeriumType::Reference(struct_type) = r#type else {
                Err(CannotReadFieldsOnType {
                    range: self.structure.0.clone(),
                    r#type: r#type.clone(),
                })?
            };
            let CeriumType::Struct(struct_name) = struct_type.as_ref() else {
                Err(CannotReadFieldsOnType {
                    range: self.structure.0.clone(),
                    r#type: r#type.clone(),
                })?
            };
            let (offset, field_type) = ctx
                .field_offset_and_type(struct_name, &self.field.1)
                .ok_or_else(|| CouldNotResolveField {
                    maybe_struct_type: struct_type.as_ref().clone(),
                    name: self.field.clone(),
                })?;
            ctx.push_inst(inst!(Mov, op operand.clone(), op op.clone()));
            ctx.push_inst(inst!(Lookup, op operand.clone(), val offset as u16));
            result = MaybeUninit::new(field_type);
            Ok(())
        })?;

        Ok(unsafe { result.assume_init() })
    }
}
