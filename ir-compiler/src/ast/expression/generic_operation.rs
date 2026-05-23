use crate::ast::compilation::context::Context;
use crate::ast::compilation::Compilable;
use crate::ast::expression::Expression;
use crate::ast::CeriumType;
use crate::error::{CompilerResult, IncompatibleTypes, UnprocessableUnit};
use crate::ranged::Ranged;
use crate::{amend, snippet};
use chasm_ir::{inst, Instruction, Operand};

#[derive(Debug, Clone, PartialEq)]
pub struct GenericOperation {
    pub lhs: Ranged<Expression>,
    pub rhs: Ranged<Expression>,
    pub operator: Ranged<GenericOperator>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GenericOperator {
    Add,
    Sub,
    Mul,
    Div,
}

impl From<GenericOperation> for Expression {
    fn from(generic_operation: GenericOperation) -> Self {
        Expression::GenericOperation(Box::new(generic_operation))
    }
}

impl Compilable for GenericOperation {
    fn compile(
        &self,
        ctx: &mut Context,
    ) -> CompilerResult<(Vec<Instruction>, Option<(Operand, CeriumType)>)> {
        self.compile_mut(ctx)
    }

    fn compile_mut(
        &self,
        ctx: &mut Context,
    ) -> CompilerResult<(Vec<Instruction>, Option<(Operand, CeriumType)>)> {
        ctx.push_scope();
        let (lhs_code, lhs_op_type) = self.lhs.1.compile_mut(ctx)?;
        let (lhs_op, lhs_type) = lhs_op_type.ok_or_else(|| UnprocessableUnit {
            range: self.lhs.0.clone(),
        })?;

        let (rhs_code, rhs_op_type) = self.rhs.1.compile(ctx)?;
        let (rhs_op, rhs_type) = rhs_op_type.ok_or_else(|| UnprocessableUnit {
            range: self.rhs.0.clone(),
        })?;
        ctx.pop_scope();

        let inst = generate_inst_for(
            self.operator.1,
            (lhs_op.clone(), &lhs_type),
            (rhs_op, &rhs_type),
        )
        .ok_or_else(|| IncompatibleTypes {
            lhs: (self.lhs.0.clone(), lhs_type.clone()),
            rhs: (self.rhs.0.clone(), rhs_type),
        })?;
        Ok((
            amend!(lhs_code, amend!(rhs_code, snippet!(inst))),
            Some((lhs_op, lhs_type)),
        ))
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<Vec<Instruction>> {
        ctx.push_scope();
        let lhs = self.lhs.1.compile_unit(ctx)?;
        ctx.pop_scope();
        ctx.push_scope();
        let rhs = self.rhs.1.compile_unit(ctx)?;
        ctx.pop_scope();
        Ok(snippet!(lhs, rhs))
    }

    fn compile_into(
        &self,
        ctx: &mut Context,
        operand: Operand,
    ) -> CompilerResult<(Vec<Instruction>, Option<CeriumType>)> {
        ctx.push_scope();
        let (lhs_code, lhs_type) = self.lhs.1.compile_into(ctx, operand.clone())?;
        let lhs_type = lhs_type.ok_or_else(|| UnprocessableUnit {
            range: self.lhs.0.clone(),
        })?;
        ctx.pop_scope();
        ctx.push_scope();
        let (rhs_code, rhs_op_type) = self.rhs.1.compile(ctx)?;
        let (rhs_op, rhs_type) = rhs_op_type.ok_or_else(|| UnprocessableUnit {
            range: self.rhs.0.clone(),
        })?;
        ctx.pop_scope();
        let inst = generate_inst_for(self.operator.1, (operand, &lhs_type), (rhs_op, &rhs_type))
            .ok_or_else(|| IncompatibleTypes {
                lhs: (self.lhs.0.clone(), lhs_type.clone()),
                rhs: (self.rhs.0.clone(), rhs_type),
            })?;
        Ok((snippet!(lhs_code, rhs_code, inst), Some(lhs_type)))
    }
}

fn generate_inst_for(
    operator: GenericOperator,
    lhs: (Operand, &CeriumType),
    rhs: (Operand, &CeriumType),
) -> Option<Instruction> {
    use CeriumType as T;
    use GenericOperator as O;

    match (operator, lhs.1, rhs.1) {
        (O::Add, T::F16, T::F16) => Some(inst!(Fadd, op lhs.0, op rhs.0)),
        (O::Add, T::I16 | T::Reference(_), T::I16) | (O::Add, T::U16 | T::Reference(_), T::U16) => {
            Some(inst!(Add, op lhs.0, op rhs.0))
        }
        (O::Sub, T::F16, T::F16) => Some(inst!(Fsub, op lhs.0, op rhs.0)),
        (O::Sub, T::I16 | T::Reference(_), T::I16) | (O::Sub, T::U16 | T::Reference(_), T::U16) => {
            Some(inst!(Sub, op lhs.0, op rhs.0))
        }
        (O::Mul, T::F16, T::F16) => Some(inst!(Fmul, op lhs.0, op rhs.0)),
        (O::Mul, T::I16, T::I16) => Some(inst!(Imul, op lhs.0, op rhs.0)),
        (O::Mul, T::U16, T::U16) => Some(inst!(Mul, op lhs.0, op rhs.0)),
        (O::Div, T::F16, T::F16) => Some(inst!(Fdiv, op lhs.0, op rhs.0)),
        (O::Div, T::I16, T::I16) => Some(inst!(Idiv, op lhs.0, op rhs.0)),
        (O::Div, T::U16, T::U16) => Some(inst!(Div, op lhs.0, op rhs.0)),
        _ => None,
    }
}
