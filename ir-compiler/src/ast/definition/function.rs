use crate::ast::cerium_type::CeriumType;
use crate::ast::compilation::context::Context;
use crate::ast::compilation::Compilable;
use crate::ast::expression::Expression;
use crate::ast::qualifier::Qualifier;
use crate::error::{CompilerResult, FalseReturnType};
use crate::ranged::Ranged;
use crate::{amend, snippet};
use chasm_ir::{inst, Instruction, Section, Words};

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

    fn chasm_signature(&self) -> (Words, Vec<(String, Words)>) {
        (
            self.return_type
                .as_ref()
                .map(|(_, r#type)| r#type.size())
                .unwrap_or(0),
            self.parameters
                .iter()
                .map(|(name, r#type)| (name.1.to_string(), r#type.1.size()))
                .collect(),
        )
    }

    pub fn compile(&self, mut ctx: Context) -> CompilerResult<Section> {
        // <compile -> <op>>; result u0[1] { mov u0 op; ret }
        let (mut body, result_op_type) = self.body.1.compile(&mut ctx)?;
        match (&self.return_type, &result_op_type) {
            (None, None) => {}
            (Some((_, expected)), Some((_, actual))) if expected == actual => {}
            _ => {
                let actual_range = match &self.body.1 {
                    Expression::Scope(scope) => scope
                        .result
                        .as_ref()
                        .map(|(range, _)| range.clone())
                        .unwrap_or(self.body.0.clone()),
                    _ => self.body.0.clone(),
                };
                Err(FalseReturnType {
                    function: self.name.clone(),
                    expected: self.return_type.clone(),
                    actual: result_op_type
                        .as_ref()
                        .map(|(_, r#type)| (actual_range, r#type.clone())),
                })?
            }
        }
        if let Some((result_op, result_type)) = result_op_type {
            let uuid = ctx.uuid();
            body = amend!(
                body,
                snippet!(Instruction::Result(
                    uuid.clone(),
                    1,
                    snippet!(inst!(Mov, uuid, op result_op))
                ))
            );
        }
        body = amend!(body, snippet!(inst!(Ret)));
        Ok(Section {
            name: self.name.1.to_string(),
            signature: todo!(),
            body,
        })
    }
}
