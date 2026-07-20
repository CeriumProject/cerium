use crate::ast::compilation::context::Context;
use crate::ast::compilation::{Compilable, ConstCompilable, ConstContext};
use crate::ast::optimize::{OptimizeExpression, OptimizeRangedExpression};
use crate::ast::{CeriumType, Expression, Qualifier};
use crate::error::{CompilerResult, InvalidGenericAmount, InvalidTurbofishType};
use crate::ranged::Ranged;
use chasm_ir::Operand;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Turbofish {
    pub generics: Ranged<Vec<Ranged<CeriumType>>>,
    pub value: Ranged<Expression>,
}

fn map_type(original: CeriumType, map: &HashMap<&Qualifier, &CeriumType>) -> CeriumType {
    match original {
        CeriumType::Struct(ref name) => match map.get(&name) {
            Some(r#type) => (*r#type).clone(),
            None => original,
        },
        CeriumType::Function(param_types, return_type) => {
            let param_types = param_types.into_iter().map(|p| map_type(p, map)).collect();
            let return_type = match return_type {
                Some(r#type) => Some(Box::new(map_type(*r#type, map))),
                None => None,
            };
            CeriumType::Function(param_types, return_type)
        }
        CeriumType::Reference(box inner) => CeriumType::Reference(Box::new(map_type(inner, map))),
        CeriumType::GenericFunction(_, _, _) => todo!(""),
        _ => original,
    }
}

fn apply_turbofish(
    generic_type: Ranged<&CeriumType>,
    generics: &Ranged<Vec<Ranged<CeriumType>>>,
) -> CompilerResult<CeriumType> {
    let CeriumType::Reference(box CeriumType::GenericFunction(
        generic_names,
        function_params,
        function_result,
    )) = generic_type.1
    else {
        Err(InvalidTurbofishType {
            range: generic_type.0,
            r#type: generic_type.1.clone(),
        })?
    };
    if generic_names.len() != generics.1.len() {
        Err(InvalidGenericAmount {
            range: generics.0.clone(),
            expected: generic_names.len(),
            supplied: generics.1.len(),
        })?
    }
    let map = generic_names
        .iter()
        .zip(generics.1.iter().map(|(_, g)| g))
        .collect::<HashMap<_, _>>();
    let function_params = function_params
        .into_iter()
        .map(|name| map_type(name.clone(), &map))
        .collect::<Vec<_>>();
    let function_result = match function_result {
        Some(function_result) => Some(Box::new(map_type(function_result.as_ref().clone(), &map))),
        None => None,
    };
    Ok(CeriumType::Reference(Box::new(CeriumType::Function(
        function_params,
        function_result,
    ))))
}

impl Compilable for Turbofish {
    fn compile(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        self.value.compile(ctx, &mut |op, r#type, ctx| {
            let r#type = apply_turbofish((self.value.0.clone(), r#type), &self.generics)?;
            then(op, &r#type, ctx)
        })
    }

    fn compile_mut(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        self.value.compile_mut(ctx, &mut |op, r#type, ctx| {
            let r#type = apply_turbofish((self.value.0.clone(), r#type), &self.generics)?;
            then(op, &r#type, ctx)
        })
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        self.value.compile_unit(ctx)
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        let result = self.value.compile_into(ctx, operand)?;
        let result = apply_turbofish((self.value.0.clone(), &result), &self.generics)?;
        Ok(result)
    }
}

impl ConstCompilable for Turbofish {
    fn compile_const(&self, ctx: &mut ConstContext) -> CompilerResult<(Operand, CeriumType)> {
        todo!()
    }
}

impl OptimizeExpression for Turbofish {
    fn optimize(self) -> Expression {
        Expression::Turbofish(Box::new(Turbofish {
            generics: self.generics,
            value: self.value.optimize(),
        }))
    }
}
