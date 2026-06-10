use chasm_ir::{inst, Operand};
use crate::ast::{CeriumType, Expression, Qualifier};
use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::error::CompilerResult;
use crate::ranged::Ranged;
use crate::token::Token;
use crate::unprocessable_unit;

#[derive(Debug, Clone, PartialEq)]
pub struct CompilerMacro {
    pub name: Ranged<Qualifier>,
    pub expressions: Vec<Ranged<Expression>>,
}

impl Compilable for CompilerMacro {
    fn compile(&self, ctx: &mut Context, then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>) -> CompilerResult<()> {
        unprocessable_unit!()
    }

    fn compile_mut(&self, ctx: &mut Context, then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>) -> CompilerResult<()> {
        unprocessable_unit!()
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        match (self.name.1.to_string().as_str(), self.expressions.as_slice()) {
            ("device", [(_, idx)]) => idx.compile_into(ctx, &Operand::Variable(String::from("rd"))).map(|_| ()),
            ("context", [(_, idx), (_, val)]) => idx.compile(ctx, &mut |idx_op, _, ctx| {
                idx.compile(ctx, &mut |val_op, _, ctx| {
                    ctx.push_inst(inst!(Ctx, op idx_op.clone(), op val_op.clone()));
                    Ok(())
                })
            }),
            ("send", []) => {
                ctx.push_inst(inst!(Send));
                Ok(())
            },
            ("dbg", values) => {
                for value in values {
                    value.1.compile(ctx, &mut |op, _, ctx| {
                        ctx.push_inst(inst!(Dbg, op op.clone()));
                        Ok(())
                    })?
                }
                Ok(())
            },
            _ => todo!()
        }
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        unprocessable_unit!()
    }
}