use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::optimize::{OptimizeExpression, OptimizeRangedExpression};
use crate::ast::{CeriumType, Expression};
use crate::error::{CompilerResult, IncompatibleBitwiseOperationTypes};
use crate::ranged::Ranged;
use chasm_ir::{Instruction, Operand, TwoOpOpcode};

#[derive(Debug, Clone, PartialEq)]
pub struct BitwiseOperation {
    pub lhs: Ranged<Expression>,
    pub rhs: Ranged<Expression>,
    pub operator: Ranged<BitwiseOperator>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BitwiseOperator {
    And,
    Or,
    Xor,
    LeftShift,
    RightShift,
}

impl BitwiseOperator {
    fn to_opcode(&self) -> TwoOpOpcode {
        match self {
            BitwiseOperator::And => TwoOpOpcode::And,
            BitwiseOperator::Or => TwoOpOpcode::Or,
            BitwiseOperator::Xor => TwoOpOpcode::Xor,
            BitwiseOperator::LeftShift => TwoOpOpcode::Shl,
            BitwiseOperator::RightShift => TwoOpOpcode::Shr,
        }
    }
}

impl Compilable for BitwiseOperation {
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
            let lhs_op = ctx.push_var(uuid.clone(), CeriumType::U16);
            let lhs_type = self.compile_into(ctx, &lhs_op)?;
            then(&lhs_op, &lhs_type, ctx)
        })
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        self.lhs.compile_unit(ctx)?;
        self.rhs.compile_unit(ctx)
    }

    fn compile_into(&self, ctx: &mut Context, lhs_op: &Operand) -> CompilerResult<CeriumType> {
        let lhs_type = self.lhs.compile_into(ctx, lhs_op)?;
        self.rhs.compile(ctx, &mut |rhs_op, rhs_type, ctx| {
            if lhs_type != CeriumType::U16 || *rhs_type != CeriumType::U16 {
                Err(IncompatibleBitwiseOperationTypes {
                    lhs: (self.lhs.0.clone(), lhs_type.clone()),
                    rhs: (self.rhs.0.clone(), rhs_type.clone()),
                })?
            };
            let opcode = self.operator.1.to_opcode();
            ctx.push_inst(Instruction::TwoOp(opcode, lhs_op.clone(), rhs_op.clone()));
            Ok(())
        })?;
        Ok(CeriumType::U16)
    }
}

impl OptimizeExpression for BitwiseOperation {
    fn optimize(self) -> Expression {
        Expression::BitwiseOperation(Box::new(BitwiseOperation {
            lhs: self.lhs.optimize(),
            rhs: self.rhs.optimize(),
            operator: self.operator,
        }))
    }
}
