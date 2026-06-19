use crate::ast::cerium_type::CeriumType;
use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::expression::Expression;
use crate::ast::qualifier::Qualifier;
use crate::error::{CompilerResult, FalseReturnType};
use crate::ranged::Ranged;
use chasm_ir::{Section, Words, inst};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: Ranged<Qualifier>,
    pub parameters: Vec<(Ranged<Qualifier>, Ranged<CeriumType>)>,
    pub return_type: Option<Ranged<CeriumType>>,
    pub body: Ranged<Expression>,
}

impl Function {
    pub fn signature(&self) -> CeriumType {
        let parameters = self
            .parameters
            .iter()
            .map(|(_, (_, r#type))| r#type.clone())
            .collect();
        let return_type = self
            .return_type
            .as_ref()
            .map(|(_, r#type)| Box::new(r#type.clone()));
        CeriumType::Reference(Box::new(CeriumType::Function(parameters, return_type)))
    }

    fn chasm_signature(
        &self,
        structs: &HashMap<Qualifier, Vec<(Qualifier, CeriumType)>>,
    ) -> CompilerResult<(Words, Vec<(String, Words)>)> {
        Ok((
            self.return_type
                .as_ref()
                .map(|(_, r#type)| r#type.size(structs))
                .unwrap_or(Ok(0))?,
            self.parameters
                .iter()
                .map(|(name, r#type)| Ok((name.1.to_string(), r#type.1.size(structs)?)))
                .collect::<CompilerResult<_>>()?,
        ))
    }

    pub fn compile(
        &self,
        globals: &HashMap<Qualifier, CeriumType>,
        structs: &HashMap<Qualifier, Vec<(Qualifier, CeriumType)>>,
    ) -> CompilerResult<Vec<Section>> {
        let parameters = self
            .parameters
            .iter()
            .map(|((_, name), (_, r#type))| (name.clone(), r#type.clone()))
            .collect();
        let mut ctx = Context::new(globals.clone(), parameters, &structs);
        // TODO: proper return type checks (None if should be Some and vise-versa)
        match &self.return_type {
            None => {
                self.body.1.compile_unit(&mut ctx)?;
                ctx.push_inst(inst!(Ret));
            }
            Some((_, expected_type)) => {
                self.body.1.compile(&mut ctx, &mut |op, actual_type, ctx| {
                    if *expected_type != *actual_type {
                        Err(FalseReturnType {
                            function: self.name.clone(),
                            expected: self.return_type.clone(),
                            actual: Some((todo!(), actual_type.clone())),
                        })?
                    }
                    let uuid = ctx.uuid();
                    let result = ctx.push_result(uuid, expected_type.clone());
                    ctx.push_inst(inst!(Mov, op result, op op.clone()));
                    ctx.push_inst(inst!(Ret));
                    Ok(())
                })?;
            }
        }

        let body = ctx.resolve()?; // TODO: ctx.section(|...| ...) instead
        Ok(vec![Section {
            name: self.name.1.to_string(),
            signature: Some(self.chasm_signature(structs)?),
            body,
        }])
    }
}
