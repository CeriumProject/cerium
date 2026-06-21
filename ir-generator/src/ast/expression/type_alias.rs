use crate::ast::CeriumType;
use crate::ast::compilation::context::Context;
use crate::ast::compilation::{Compilable, ConstCompilable, ConstContext};
use crate::ast::expression::Expression;
use crate::ast::expression::optimize::{OptimizeExpression, OptimizeRangedExpression};
use crate::error::{CompilerResult, TypeAliasHasDifferentSize};
use crate::ranged::Ranged;
use chasm_ir::Operand;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAlias {
    pub value: Ranged<Expression>,
    pub r#type: Ranged<CeriumType>,
}

impl Compilable for TypeAlias {
    fn compile(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        self.value.1.compile(ctx, &mut |op, prev_type, ctx| {
            let new_type = &self.r#type.1;
            if prev_type.size(ctx.structs()) != new_type.size(ctx.structs()) {
                Err(TypeAliasHasDifferentSize {
                    source: (self.value.0.clone(), prev_type.clone()),
                    target: self.r#type.clone(),
                })?
            }
            then(op, &self.r#type.1, ctx)
        })
    }

    fn compile_mut(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        self.value.1.compile_mut(ctx, &mut |op, prev_type, ctx| {
            let new_type = &self.r#type.1;
            if prev_type.size(ctx.structs()) != new_type.size(ctx.structs()) {
                Err(TypeAliasHasDifferentSize {
                    source: (self.value.0.clone(), prev_type.clone()),
                    target: self.r#type.clone(),
                })?
            }
            then(op, &self.r#type.1, ctx)
        })
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        self.value.1.compile_unit(ctx)
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        let prev_type = self.value.1.compile_into(ctx, operand)?;
        let new_type = self.r#type.1.clone();
        if prev_type.size(ctx.structs()) != new_type.size(ctx.structs()) {
            Err(TypeAliasHasDifferentSize {
                source: (self.value.0.clone(), prev_type.clone()),
                target: self.r#type.clone(),
            })?
        }
        Ok(new_type)
    }
}

impl ConstCompilable for TypeAlias {
    fn compile_const(&self, ctx: &mut ConstContext) -> CompilerResult<(Operand, CeriumType)> {
        let (op, _) = self.value.1.compile_const(ctx)?;
        Ok((op, self.r#type.1.clone()))
    }
}

impl OptimizeExpression for TypeAlias {
    fn optimize(self) -> Expression {
        Expression::TypeAlias(Box::new(TypeAlias {
            value: self.value.optimize(),
            r#type: self.r#type,
        }))
    }
}
