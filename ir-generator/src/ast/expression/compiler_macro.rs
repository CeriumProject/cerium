use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::expression::optimize::{OptimizeExpression, OptimizeRangedExpression};
use crate::ast::{CeriumType, Expression, Qualifier};
use crate::error::CompilerResult;
use crate::ranged::Ranged;
use crate::unprocessable_unit;
use chasm_ir::{Operand, inst};

#[derive(Debug, Clone, PartialEq)]
pub struct CompilerMacro {
    pub name: Ranged<Qualifier>,
    pub expressions: Vec<Ranged<Expression>>,
}

impl Compilable for CompilerMacro {
    fn compile(
        &self,
        _ctx: &mut Context,
        _then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        unprocessable_unit!()
    }

    fn compile_mut(
        &self,
        _ctx: &mut Context,
        _then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        unprocessable_unit!()
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        match (
            self.name.1.to_string().as_str(),
            self.expressions.as_slice(),
        ) {
            ("device", [(_, idx)]) => idx
                .compile_into(ctx, &Operand::Variable(String::from("rd")))
                .map(|_| ()),
            ("context", [(_, idx), (_, val)]) => idx.compile(ctx, &mut |idx_op, _, ctx| {
                val.compile(ctx, &mut |val_op, _, ctx| {
                    ctx.push_inst(inst!(Ctx, op idx_op.clone(), op val_op.clone()));
                    Ok(())
                })
            }),
            ("send", []) => {
                ctx.push_inst(inst!(Send));
                Ok(())
            }
            ("dbg", values) => {
                for value in values {
                    value.1.compile(ctx, &mut |op, _, ctx| {
                        ctx.push_inst(inst!(Dbg, op op.clone()));
                        Ok(())
                    })?
                }
                Ok(())
            }
            _ => todo!(),
        }
    }

    fn compile_into(&self, _ctx: &mut Context, _operand: &Operand) -> CompilerResult<CeriumType> {
        unprocessable_unit!()
    }
}

impl OptimizeExpression for CompilerMacro {
    fn optimize(self) -> Expression {
        let expressions = self
            .expressions
            .into_iter()
            .map(Ranged::<Expression>::optimize)
            .collect();
        Expression::CompilerMacro(Box::new(CompilerMacro {
            name: self.name,
            expressions,
        }))
    }
}
