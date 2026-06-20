use crate::ast::compilation::context::Context;
use crate::ast::compilation::{Compilable, ConstCompilable, ConstContext};
use crate::ast::struct_initialization::StructInitialization;
use crate::ast::{Array, CeriumType, Expression};
use crate::error::{
    CompilerResult, CouldNotResolveType, FalseFieldType, MismatchedAssignmentType, UnassignedField,
    ValueNotReferenceable,
};
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

impl ConstCompilable for Reference {
    fn compile_const(&self, ctx: &mut ConstContext) -> CompilerResult<(Operand, CeriumType)> {
        let (ops, r#type) = match &self.inner.1 {
            Expression::Array(box Array { elements }) => {
                let (ops, types) = elements
                    .iter()
                    .map(|(_, expression)| expression.compile_const(ctx))
                    .try_fold((Vec::new(), Vec::new()), |mut result, op_type| {
                        result.extend([op_type?]);
                        CompilerResult::<(Vec<_>, Vec<_>)>::Ok(result)
                    })?;
                let r#type = match types.as_slice() {
                    [] => todo!("error"),
                    [former_type, rest @ ..] => {
                        let mismatches = rest
                            .iter()
                            .enumerate()
                            .filter(|(_, latter_type)| **latter_type != *former_type)
                            .collect::<Vec<_>>();
                        if mismatches.is_empty() {
                            former_type.clone()
                        } else {
                            dbg!(mismatches);
                            todo!("error")
                        }
                    }
                };
                (ops, r#type)
            }
            Expression::StructInitialization(box StructInitialization { name, fields }) => {
                let struct_fields = ctx
                    .lookup_struct(&name.1)
                    .ok_or_else(|| CouldNotResolveType { name: name.clone() })?;
                let mut words = Vec::new();
                for (field_name, field_type) in struct_fields {
                    let ((range, _), value) = fields
                        .iter()
                        .find(|((_, name), _)| *name == *field_name)
                        .ok_or_else(|| UnassignedField {
                            range: self.inner.0.clone(),
                            field: field_name.clone(),
                        })?;
                    let (op, r#type) = value.1.compile_const(unsafe {
                        #[allow(mutable_transmutes)]
                        std::mem::transmute::<&_, &mut _>(ctx)
                    })?;
                    if *field_type != r#type {
                        Err(FalseFieldType {
                            field: (range.clone(), field_name.clone()),
                            expected: field_type.clone(),
                            actual: r#type,
                        })?
                    }
                    words.push(op);
                }
                (words, CeriumType::Struct(name.1.clone()))
            }
            expression => {
                let (op, r#type) = expression.compile_const(ctx)?;
                (vec![op], r#type)
            }
        };
        let uuid = ctx.push_section(vec![Instruction::RawWords(ops)]);
        Ok((uuid, CeriumType::Reference(Box::new(r#type))))
    }
}
