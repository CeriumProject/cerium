use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::expression::optimize::{OptimizeExpression, OptimizeRangedExpression};
use crate::ast::{CeriumType, Expression, Qualifier};
use crate::error::{
    CompilerResult, InvalidParameterAmount, MismatchedParameterType, ValueNotInvocable,
};
use crate::ranged::Ranged;
use crate::unprocessable_unit;
use chasm_ir::{Instruction, Operand, inst};
use std::collections::HashMap;
use std::mem::MaybeUninit;

#[derive(Debug, Clone, PartialEq)]
pub struct Invocation {
    pub function: Ranged<Expression>,
    pub parameters: Vec<Ranged<Expression>>,
}

impl Invocation {
    fn compile_params(&self, ctx: &mut Context) -> CompilerResult<Vec<Ranged<CeriumType>>> {
        self.parameters
            .iter()
            .map(|(range, parameter)| {
                let uuid = ctx.uuid();
                let op = ctx.push_param(uuid.clone(), CeriumType::I16); // placeholder type
                let r#type = parameter.compile_into(ctx, &op)?;
                ctx.change_type(&Qualifier::short(uuid), r#type.clone());
                Ok((range.clone(), r#type))
            })
            .collect()
    }
}

impl Compilable for Invocation {
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
            let param_types = self.compile_params(ctx)?;
            self.function.1.compile(ctx, &mut |function, r#type, ctx| {
                ctx.push_inst(inst!(Call, op function.clone()));
                let result_type = check_parameter_types(
                    &param_types,
                    (self.function.0.clone(), r#type),
                    ctx.structs(),
                )?;
                let Some(result_type) = result_type else {
                    unprocessable_unit!();
                };
                ctx.scope(|ctx| {
                    let uuid = ctx.uuid();
                    let op = ctx.push_var(uuid, result_type.clone());
                    ctx.push_inst(Instruction::Receive(op.clone(), 0));
                    then(&op, &result_type, ctx)
                })
            })
        })
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        ctx.scope(|ctx| {
            let param_types = self.compile_params(ctx)?;
            self.function.1.compile(ctx, &mut |function, r#type, ctx| {
                ctx.push_inst(inst!(Call, op function.clone()));
                check_parameter_types(
                    &param_types,
                    (self.function.0.clone(), r#type),
                    ctx.structs(),
                )?;
                Ok(())
            })
        })
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        ctx.scope(|ctx| {
            let param_types = self.compile_params(ctx)?;
            let mut outer_result_type = MaybeUninit::uninit();
            self.function.1.compile(ctx, &mut |function, r#type, ctx| {
                ctx.push_inst(inst!(Call, op function.clone()));
                let result_type = check_parameter_types(
                    &param_types,
                    (self.function.0.clone(), r#type),
                    ctx.structs(),
                )?;
                let Some(result_type) = result_type else {
                    unprocessable_unit!();
                };
                ctx.push_inst(Instruction::Receive(operand.clone(), 0));
                outer_result_type = MaybeUninit::new(result_type);
                Ok(())
            })?;
            Ok(unsafe { outer_result_type.assume_init() })
        })
    }
}

fn check_parameter_types(
    param_types: &[Ranged<CeriumType>],
    function_type: Ranged<&CeriumType>,
    structs: &HashMap<Qualifier, Vec<(Qualifier, CeriumType)>>,
) -> CompilerResult<Option<CeriumType>> {
    match function_type {
        (range, CeriumType::Reference(inner)) => {
            let CeriumType::Function(parameters, result) = inner.as_ref() else {
                Err(ValueNotInvocable {
                    range,
                    r#type: inner.as_ref().clone(),
                })?
            };
            if parameters.len() != param_types.len() {
                Err(InvalidParameterAmount {
                    function: range,
                    expected: parameters.len(),
                    supplied: param_types.len(),
                })?
            }
            for (lhs, (range, rhs)) in parameters.iter().zip(param_types.iter()) {
                if !rhs.is_subtype_of(lhs, structs)? {
                    Err(MismatchedParameterType {
                        parameter: range.clone(),
                        expected: lhs.clone(),
                        supplied: rhs.clone(),
                    })?
                }
            }
            Ok(result
                .as_ref()
                .map(|return_type| return_type.as_ref().clone()))
        }
        (range, actual_type) => Err(ValueNotInvocable {
            range,
            r#type: actual_type.clone(),
        })?,
    }
}

impl OptimizeExpression for Invocation {
    fn optimize(self) -> Expression {
        Expression::Invocation(Box::new(Invocation {
            function: self.function.optimize(),
            parameters: self
                .parameters
                .into_iter()
                .map(Ranged::<Expression>::optimize)
                .collect(),
        }))
    }
}
