use crate::ast::CeriumType;
use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::expression::Expression;
use crate::ast::expression::optimize::{OptimizeExpression, OptimizeRangedExpression};
use crate::ast::qualifier::Qualifier;
use crate::error::{CompilerResult, MismatchedAssignmentType};
use crate::ranged::Ranged;
use crate::unprocessable_unit;
use chasm_ir::Operand;

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub name: Ranged<Qualifier>,
    pub r#type: Option<Ranged<CeriumType>>,
    pub value: Ranged<Expression>,
}

impl Compilable for Declaration {
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
        // placeholder
        let initial_type = match &self.r#type {
            Some(r#type) => r#type.1.clone(),
            None => CeriumType::Any(1),
        };
        let op = ctx.push_var(self.name.1.clone(), initial_type.clone());

        let r#type = self.value.1.compile_into(ctx, &op)?;
        if !r#type.is_subtype_of(&initial_type, ctx.structs())? {
            Err(MismatchedAssignmentType {
                destination: (self.name.0.clone(), initial_type.clone()),
                source: (self.value.0.clone(), r#type.clone()),
            })?
        }
        if self.r#type.is_none() {
            ctx.change_type(&self.name.1, r#type);
        }
        Ok(())
    }

    fn compile_into(&self, _ctx: &mut Context, _operand: &Operand) -> CompilerResult<CeriumType> {
        unprocessable_unit!()
    }
}

impl OptimizeExpression for Declaration {
    fn optimize(self) -> Expression {
        Expression::Declaration(Box::new(Declaration {
            name: self.name,
            r#type: self.r#type,
            value: self.value.optimize(),
        }))
    }
}
