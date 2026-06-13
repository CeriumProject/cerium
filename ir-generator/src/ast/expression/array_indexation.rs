use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::{CeriumType, Expression};
use crate::error::{CompilerResult, IndexMustBeInteger, ValueNotDereferenceable};
use crate::ranged::Ranged;
use chasm_ir::{Operand, inst};

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayIndexation {
    pub array: Ranged<Expression>,
    pub index: Ranged<Expression>,
}

impl Compilable for ArrayIndexation {
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
        self.array.1.compile_mut(ctx, &mut |arr_op, arr_type, ctx| {
            let CeriumType::Reference(inner_type) = arr_type else {
                Err(ValueNotDereferenceable {
                    range: self.array.0.clone(),
                    r#type: arr_type.clone(),
                })?
            };
            self.index.1.compile(ctx, &mut |idx_op, idx_type, ctx| {
                let (CeriumType::I16 | CeriumType::U16) = idx_type else {
                    Err(IndexMustBeInteger {
                        range: self.index.0.clone(),
                        encountered: idx_type.clone(),
                    })?
                };
                ctx.push_inst(inst!(Lookup, op arr_op.clone(), op idx_op.clone()));
                Ok(())
            })?;
            then(arr_op, inner_type.as_ref(), ctx)
        })
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        self.array.1.compile_unit(ctx)?;
        self.index.1.compile_unit(ctx)
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        let array_type = self.array.1.compile_into(ctx, operand)?;
        let CeriumType::Reference(inner_type) = array_type else {
            Err(ValueNotDereferenceable {
                range: self.array.0.clone(),
                r#type: array_type,
            })?
        };
        self.index.1.compile(ctx, &mut |op, r#type, ctx| {
            let (CeriumType::I16 | CeriumType::U16) = r#type else {
                Err(IndexMustBeInteger {
                    range: self.index.0.clone(),
                    encountered: r#type.clone(),
                })?
            };
            ctx.push_inst(inst!(Lookup, op operand.clone(), op op.clone()));
            Ok(())
        })?;
        Ok(*inner_type)
    }
}
