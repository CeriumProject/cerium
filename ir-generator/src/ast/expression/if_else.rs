use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::optimize::{OptimizeExpression, OptimizeRangedExpression};
use crate::ast::scope::peek_scope_rec;
use crate::ast::{CeriumType, Expression, Qualifier};
use crate::error::{CompilerResult, ConditionMustBeBool, MismatchedBranchTypes};
use crate::ranged::Ranged;
use chasm_ir::{Instruction, Operand, inst};
use std::ops::Add;

#[derive(Debug, Clone, PartialEq)]
pub struct IfElse {
    pub condition: Ranged<Expression>,
    pub if_body: Ranged<Expression>,
    pub else_body: Option<Ranged<Expression>>,
}

impl From<IfElse> for Expression {
    fn from(value: IfElse) -> Self {
        Expression::IfElse(Box::new(value))
    }
}

impl Compilable for IfElse {
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
        ctx.scope(|ctx| {
            let uuid = ctx.uuid();
            let op = ctx.push_var(uuid.clone(), CeriumType::Undefined(1));
            let r#type = self.compile_into(ctx, &op)?;
            ctx.change_type(&Qualifier::from(uuid.clone()), r#type.clone());
            then(&op, &r#type, ctx)
        })
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        /*
            <condition -> var>
            jrz var .L0
            <if-branch>
            jmp .L1
        .L0:
            <else-branch>
        .L1:
         */
        let else_label = ctx.label();
        let end_label = ctx.label();
        self.condition.compile(ctx, &mut |op, r#type, ctx| {
            if *r#type != CeriumType::Bool {
                Err(ConditionMustBeBool {
                    condition_range: self.condition.0.clone(),
                    encountered_type: r#type.clone(),
                })?;
            }
            ctx.push_inst(inst!(Jrz, op op.clone(), format!(".{}", &else_label)));
            Ok(())
        })?;
        self.if_body.compile_unit(ctx)?;
        if let Some(else_body) = &self.else_body {
            ctx.push_inst(inst!(Jmp, format!(".{}", &end_label)));
            ctx.push_inst(Instruction::Sublabel(else_label));
            else_body.compile_unit(ctx)?;
            ctx.push_inst(Instruction::Sublabel(end_label));
        } else {
            ctx.push_inst(Instruction::Sublabel(else_label));
        }
        Ok(())
    }

    fn compile_into(&self, ctx: &mut Context, target: &Operand) -> CompilerResult<CeriumType> {
        let else_label = ctx.label();
        let end_label = ctx.label();
        self.condition.compile(ctx, &mut |op, r#type, ctx| {
            if *r#type != CeriumType::Bool {
                Err(ConditionMustBeBool {
                    condition_range: self.condition.0.clone(),
                    encountered_type: r#type.clone(),
                })?;
            }
            ctx.push_inst(inst!(Jrz, op op.clone(), format!(".{}", &else_label)));
            Ok(())
        })?;
        let if_type = self.if_body.compile_into(ctx, target)?;
        let else_type = if let Some(else_body) = &self.else_body {
            ctx.push_inst(inst!(Jmp, format!(".{}", &end_label)));
            ctx.push_inst(Instruction::Sublabel(else_label));
            let else_type = else_body.compile_into(ctx, target)?;
            ctx.push_inst(Instruction::Sublabel(end_label));
            else_type
        } else {
            ctx.push_inst(Instruction::Sublabel(else_label));
            CeriumType::Undefined(0)
        };
        if if_type != else_type {
            let else_range = match &self.else_body {
                Some(else_body) => peek_scope_rec(else_body).0.clone(),
                None => {
                    let idx = self.if_body.0.end().add(1);
                    idx..=idx
                }
            };
            Err(MismatchedBranchTypes {
                if_type: (peek_scope_rec(&self.if_body).0.clone(), if_type),
                else_type: (else_range, else_type),
            })?
        } else {
            Ok(if_type)
        }
    }
}

impl OptimizeExpression for IfElse {
    fn optimize(self) -> Expression {
        // TODO: try compile_const in condition to check if it is always true/false
        Expression::IfElse(Box::new(IfElse {
            condition: self.condition.optimize(),
            if_body: self.if_body.optimize(),
            else_body: self.else_body.map(OptimizeRangedExpression::optimize),
        }))
    }
}
