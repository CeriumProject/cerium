use crate::ast::compilation::context::Context;
use crate::ast::compilation::{Compilable, ConstCompilable, ConstContext};
use crate::ast::expression::optimize::OptimizeExpression;
use crate::ast::{CeriumType, Expression};
use crate::error::{CompilerResult, UnparseableConstant};
use crate::ranged::Ranged;
use chasm_ir::{Operand, inst};

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantValue {
    pub value: Ranged<String>,
}

impl ConstantValue {
    fn parse(&self) -> CompilerResult<(u16, CeriumType)> {
        let (range, raw_constant) = self.value.clone();

        match raw_constant.as_str() {
            "true" => return Ok((1, CeriumType::Bool)),
            "false" => return Ok((0, CeriumType::Bool)),
            _ => {}
        }

        if raw_constant.contains('.') {
            raw_constant
                .parse::<f16>()
                .map(|f| (f.to_bits(), CeriumType::F16))
                .map_err(|_| {
                    UnparseableConstant {
                        raw_constant,
                        range,
                    }
                    .into()
                })
        } else if raw_constant.starts_with('-') || raw_constant.starts_with('+') {
            raw_constant
                .parse::<i16>()
                .map(|i| (i as u16, CeriumType::I16))
                .map_err(|_| {
                    UnparseableConstant {
                        raw_constant,
                        range,
                    }
                    .into()
                })
        } else {
            raw_constant
                .parse::<u16>()
                .map(|u| (u, CeriumType::U16))
                .map_err(|_| {
                    UnparseableConstant {
                        raw_constant,
                        range,
                    }
                    .into()
                })
        }
    }
}

impl Compilable for ConstantValue {
    fn compile(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        let (val, ty) = self.parse()?;
        then(&Operand::Constant(val), &ty, ctx)
    }

    fn compile_mut(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        ctx.scope(|ctx| {
            let (val, ty) = self.parse()?;
            let uuid = ctx.uuid();
            let op = ctx.push_var(uuid, ty.clone());
            ctx.push_inst(inst!(Mov, op op.clone(), val val));
            then(&op, &ty, ctx)
        })
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        Ok(())
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        let (val, ty) = self.parse()?;
        ctx.push_inst(inst!(Mov, op operand.clone(), val val));
        Ok(ty)
    }
}

impl ConstCompilable for ConstantValue {
    fn compile_const(&self, ctx: &mut ConstContext) -> CompilerResult<(Operand, CeriumType)> {
        self.parse()
            .map(|(val, r#type)| (Operand::Constant(val), r#type))
    }
}

impl OptimizeExpression for ConstantValue {
    fn optimize(self) -> Expression {
        Expression::Constant(Box::new(self))
    }
}
