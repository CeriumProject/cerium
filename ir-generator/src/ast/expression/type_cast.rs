use crate::ast::CeriumType;
use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::expression::Expression;
use crate::ast::expression::optimize::{OptimizeExpression, OptimizeRangedExpression};
use crate::error::{CannotCastType, CompilerResult};
use crate::ranged::Ranged;
use chasm_ir::{Instruction, Operand, TwoOpOpcode};

#[derive(Debug, Clone, PartialEq)]
pub struct TypeCast {
    pub value: Ranged<Expression>,
    pub r#type: Ranged<CeriumType>,
}

impl Compilable for TypeCast {
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
        self.value.1.compile_mut(ctx, &mut |op, r#type, ctx| {
            let opcode = opcode(r#type, &self.r#type.1).ok_or_else(|| CannotCastType {
                from: (self.value.0.clone(), r#type.clone()),
                to: self.r#type.clone(),
            })?;
            ctx.push_inst(Instruction::TwoOp(opcode, op.clone(), op.clone()));
            then(op, &self.r#type.1, ctx)
        })
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        self.value.1.compile_unit(ctx)
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        self.value.1.compile(ctx, &mut |op, r#type, ctx| {
            let opcode = opcode(r#type, &self.r#type.1).ok_or_else(|| CannotCastType {
                from: (self.value.0.clone(), r#type.clone()),
                to: self.r#type.clone(),
            })?;
            ctx.push_inst(Instruction::TwoOp(opcode, operand.clone(), op.clone()));
            Ok(())
        })?;
        Ok(self.r#type.1.clone())
    }
}

fn opcode(from: &CeriumType, to: &CeriumType) -> Option<TwoOpOpcode> {
    match (from, to) {
        (from, to) if from == to => Some(TwoOpOpcode::Mov),
        (CeriumType::I16, CeriumType::F16) => Some(TwoOpOpcode::Itof),
        (CeriumType::U16, CeriumType::F16) => Some(TwoOpOpcode::Utof),
        (CeriumType::F16, CeriumType::I16) => Some(TwoOpOpcode::Ftoi),
        (CeriumType::F16, CeriumType::U16) => Some(TwoOpOpcode::Ftou),
        (CeriumType::I16, CeriumType::U16) => Some(TwoOpOpcode::Mov),
        (CeriumType::U16, CeriumType::I16) => Some(TwoOpOpcode::Mov),
        _ => None,
    }
}

impl OptimizeExpression for TypeCast {
    fn optimize(self) -> Expression {
        Expression::TypeCast(Box::new(TypeCast {
            value: self.value.optimize(),
            r#type: self.r#type,
        }))
    }
}
