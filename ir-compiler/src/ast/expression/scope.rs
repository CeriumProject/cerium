use chasm_ir::{inst, Instruction, Operand};
use crate::{amend, snippet};
use crate::ast::CeriumType;
use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::expression::Expression;
use crate::error::CompilerResult;
use crate::ranged::Ranged;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    pub statements: Vec<Ranged<Expression>>,
    pub result: Option<Ranged<Expression>>,
}

impl Compilable for Scope {
    fn compile(&self, ctx: &mut Context) -> CompilerResult<(Vec<Instruction>, Option<(Operand, CeriumType)>)> {
        let mut result = Vec::new();
        ctx.push_scope();
        for (_, statement) in &self.statements {
            result = amend!(result, statement.compile_unit(ctx)?);
        }
        let op_type = if let Some((_, statement)) = &self.result {
            let (code, op_type) = statement.compile(ctx)?;
            result = amend!(result, code);
            op_type
        } else {
            None
        };
        ctx.pop_scope();
        Ok((result, op_type))
    }

    fn compile_mut(&self, ctx: &mut Context) -> CompilerResult<(Vec<Instruction>, Option<(Operand, CeriumType)>)> {
        let mut result = Vec::new();
        ctx.push_scope();
        let uuid = ctx.uuid();
        for (_, statement) in &self.statements {
            result = amend!(result, statement.compile_unit(ctx)?);
        }
        let op_type = match &self.result {
            Some((_, statement)) => {
                let (code, op_type) = statement.compile(ctx)?;
                result = amend!(result, code);
                if let Some((op, r#type)) = op_type {
                    result = snippet!(Instruction::Alloc(uuid.clone(), 1, amend!(result, snippet!(inst!(Mov, uuid.clone(), op op)))));
                    Some((Operand::Variable(uuid), r#type))
                } else {
                    None
                }
            },
            None => None,
        };
        ctx.pop_scope();
        Ok((result, op_type))
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<Vec<Instruction>> {
        let mut result = Vec::new();
        ctx.push_scope();
        for (_, statement) in &self.statements {
            result = amend!(result, statement.compile_unit(ctx)?);
        }
        if let Some((_, statement)) = &self.result {
            result = amend!(result, statement.compile_unit(ctx)?);
        }
        ctx.pop_scope();
        Ok(result)
    }

    fn compile_into(&self, ctx: &mut Context, operand: Operand) -> CompilerResult<(Vec<Instruction>, Option<CeriumType>)> {
        todo!()
    }
}