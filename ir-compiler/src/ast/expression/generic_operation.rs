use crate::ast::CeriumType;
use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::expression::Expression;
use crate::error::{CompilerResult, IncompatibleTypes, UnprocessableUnit};
use crate::ranged::Ranged;
use chasm_ir::{Instruction, Operand, inst};

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
        then: impl FnOnce(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        self.compile_mut(ctx, then)
    }

    fn compile_mut(
        &self,
        ctx: &mut Context,
        then: impl FnOnce(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        ctx.scope(|ctx| {
            self.lhs.1.compile_mut(ctx, |lhs_op, lhs_type, ctx| {
                self.rhs.1.compile(ctx, |rhs_op, rhs_type, ctx| {
                    let inst = generate_inst_for(
                        self.operator.1,
                        (lhs_op.clone(), lhs_type),
                        (rhs_op.clone(), rhs_type),
                    )
                    .ok_or_else(|| IncompatibleTypes {
                        lhs: (self.lhs.0.clone(), lhs_type.clone()),
                        rhs: (self.rhs.0.clone(), rhs_type.clone()),
                    })?;
                    ctx.push_inst(inst);
                    Ok(())
                })
            })
        })
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        ctx.scope(|ctx| self.lhs.1.compile_unit(ctx))?;
        ctx.scope(|ctx| self.rhs.1.compile_unit(ctx))
    }

    fn compile_into(&self, ctx: &mut Context, lhs_op: &Operand) -> CompilerResult<CeriumType> {
        let lhs_type = self.lhs.1.compile_into(ctx, lhs_op)?;
        ctx.scope(|ctx| {
            self.rhs.1.compile(ctx, |rhs_op, rhs_type, ctx| {
                let inst = generate_inst_for(
                    self.operator.1,
                    (lhs_op.clone(), &lhs_type),
                    (rhs_op.clone(), rhs_type),
                )
                .ok_or_else(|| IncompatibleTypes {
                    lhs: (self.lhs.0.clone(), lhs_type.clone()),
                    rhs: (self.rhs.0.clone(), rhs_type.clone()),
                })?;
                ctx.push_inst(inst);
                Ok(())
            })
        })?;
        Ok(lhs_type)
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
