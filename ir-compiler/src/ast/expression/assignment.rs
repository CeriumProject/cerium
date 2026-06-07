use crate::ast::CeriumType;
use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::expression::Expression;
use crate::error::{CompilerResult, MismatchedAssignmentType, ValueNotDereferenceable};
use crate::ranged::Ranged;
use chasm_ir::{inst, Operand};
use crate::ast::dereference::Dereference;
use crate::unprocessable_unit;

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub dest: Ranged<Expression>,
    pub source: Ranged<Expression>,
}

/*
Cannot use compile_into for assignment because the variable might be read after updated.
```
let x = 10;
x = x + 4 + x;
```
Checking recursively for operands does not help, because pointers might be used.
```
let x = 10;
let y = &x;
x = *y + 4 + *y;
```
Solution: leave problem to ir->asm step or also check recursively for (mutable) pointers.
TODO: recurse for matching operands/variables and (mutable) references
 */
impl Compilable for Assignment {
    fn compile(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        unprocessable_unit!()
    }

    fn compile_mut(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        unprocessable_unit!()
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        self.source.1.compile(ctx, &mut |val_op, val_type, ctx| {
            match &self.dest.1 {
                Expression::Variable(variable) => {
                    variable.compile(ctx, &mut |var_op, var_type, ctx| {
                        if *val_type != *var_type {
                            Err(MismatchedAssignmentType {
                                destination: (self.dest.0.clone(), var_type.clone()),
                                source: (self.source.0.clone(), val_type.clone()),
                            })?;
                        }
                        ctx.push_inst(inst!(Mov, op var_op.clone(), op val_op.clone()));
                        Ok(())
                    })
                },
                Expression::Dereference(reference) => {
                    let Dereference { inner: (ref_range, reference) } = reference.as_ref();
                    reference.compile(ctx, &mut |var_op, var_type, ctx| {
                        let CeriumType::Reference(var_type) = var_type else {
                            Err(ValueNotDereferenceable {
                                range: ref_range.clone(),
                                r#type: var_type.clone(),
                            })?
                        };
                        if *val_type != **var_type {
                            Err(MismatchedAssignmentType {
                                destination: (self.dest.0.clone(), var_type.as_ref().clone()),
                                source: (self.source.0.clone(), val_type.clone()),
                            })?;
                        }
                        ctx.push_inst(inst!(Write, op var_op.clone(), op val_op.clone()));
                        Ok(())
                    })
                }
                _ => todo!("error"),
            }
        })
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        unprocessable_unit!()
    }
}
