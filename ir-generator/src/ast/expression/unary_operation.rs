use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::constant_value::RawConstantValue;
use crate::ast::expression::optimize::OptimizeExpression;
use crate::ast::optimize::OptimizeRangedExpression;
use crate::ast::{CeriumType, ConstantValue, Expression};
use crate::error::CompilerResult;
use crate::ranged::Ranged;
use chasm_ir::Operand;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOperation {
    pub operator: Ranged<UnaryOperator>,
    pub value: Ranged<Expression>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UnaryOperator {
    Plus,
    Minus,
}

impl Compilable for UnaryOperation {
    fn compile(
        &self,
        _ctx: &mut Context,
        _then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        todo!()
    }

    fn compile_mut(
        &self,
        _ctx: &mut Context,
        _then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        todo!()
    }

    fn compile_unit(&self, _ctx: &mut Context) -> CompilerResult<()> {
        todo!()
    }

    fn compile_into(&self, _ctx: &mut Context, _operand: &Operand) -> CompilerResult<CeriumType> {
        todo!()
    }
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Plus => write!(f, "+"),
            UnaryOperator::Minus => write!(f, "-"),
        }
    }
}

impl OptimizeExpression for UnaryOperation {
    fn optimize(self) -> Expression {
        let value = self.value.optimize();
        if let (
            _,
            Expression::Constant(box ConstantValue {
                value: (range, RawConstantValue::Number(number)),
            }),
        ) = value
        {
            // TODO: range
            let number = RawConstantValue::Number(format!("{}{}", self.operator.1, number));
            Expression::Constant(Box::new(ConstantValue {
                value: (range, number),
            }))
        } else {
            Expression::UnaryOperation(Box::new(UnaryOperation {
                operator: self.operator,
                value,
            }))
        }
    }
}
