use std::collections::HashMap;
use std::ops::Deref;
use chasm_ir::Operand;
use crate::ast::{CeriumType, Expression, Qualifier};
use crate::ast::compilation::{Compilable, ConstCompilable, ConstContext};
use crate::ast::compilation::context::Context;
use crate::ast::optimize::OptimizeExpression;
use crate::error::CompilerResult;
use crate::ranged::Ranged;

#[derive(Debug, Clone, PartialEq)]
pub struct Turbofish {
    pub generics: Vec<Ranged<CeriumType>>,
    pub value: Ranged<Expression>,
}

fn map_type(original: &CeriumType, map: &HashMap<&Qualifier, &CeriumType>) -> Option<CeriumType> {
    match original {
        CeriumType::Struct(name) => map.get(name).map(Deref::deref).unwrap_or(original).clone(),
        CeriumType::Function(param_types, return_type) => {
            
        },
        CeriumType::GenericFunction()
    }
}

fn apply_turbofish(generic_type: &CeriumType, generics: &[CeriumType]) -> Option<CeriumType> {
    let CeriumType::GenericFunction(generic_names, function_params, function_result) = generic_type else {
        return None;
    };
    if generic_names.len() != function_params.len() {
        return None;
    }
    let map = generic_names.iter().zip(function_params.iter()).collect::<HashMap<_, _>>();
    let function_params = function_params.into_iter().map(|name| match name {
        CeriumType::Struct(struct_name) => todo!(),
        Cer
    })
    
}

impl Compilable for Turbofish {
    fn compile(&self, ctx: &mut Context, then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>) -> CompilerResult<()> {
        todo!()
    }

    fn compile_mut(&self, ctx: &mut Context, then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>) -> CompilerResult<()> {
        todo!()
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        todo!()
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        todo!()
    }
}

impl ConstCompilable for Turbofish {
    fn compile_const(&self, ctx: &mut ConstContext) -> CompilerResult<(Operand, CeriumType)> {
        todo!()
    }
}

impl OptimizeExpression for Turbofish {
    fn optimize(self) -> Expression {
        todo!()
    }
}