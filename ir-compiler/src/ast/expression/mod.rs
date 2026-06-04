use crate::ast::CeriumType;
use crate::ast::compilation::Compilable;
use crate::ast::compilation::context::Context;
use crate::ast::dereference::Dereference;
pub use crate::ast::expression::assignment::Assignment;
pub use crate::ast::expression::constant_value::ConstantValue;
pub use crate::ast::expression::declaration::Declaration;
pub use crate::ast::expression::for_downto::ForDownTo;
pub use crate::ast::expression::generic_operation::GenericOperation;
pub use crate::ast::expression::plain_loop::Loop;
pub use crate::ast::expression::scope::Scope;
pub use crate::ast::expression::type_alias::TypeAlias;
pub use crate::ast::expression::type_cast::TypeCast;
pub use crate::ast::expression::variable::Variable;
pub use crate::ast::invocation::Invocation;
pub use crate::ast::reference::Reference;
use crate::error::CompilerResult;
use chasm_ir::Operand;

pub mod assignment;
pub mod constant_value;
pub mod declaration;
pub mod dereference;
pub mod for_downto;
pub mod generic_operation;
pub mod invocation;
pub mod plain_loop;
pub mod reference;
pub mod scope;
pub mod type_alias;
pub mod type_cast;
pub mod variable;

#[macro_export]
macro_rules! unprocessable_unit {
    () => {
        todo!("Return value of type unit.")
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Variable(Box<Variable>),
    Constant(Box<ConstantValue>),
    Scope(Box<Scope>),
    Declaration(Box<Declaration>),
    ForDownTo(Box<ForDownTo>),
    Loop(Box<Loop>),
    Assignment(Box<Assignment>),
    GenericOperation(Box<GenericOperation>),
    TypeCast(Box<TypeCast>),
    TypeAlias(Box<TypeAlias>),
    Reference(Box<Reference>),
    Dereference(Box<Dereference>),
    Invocation(Box<Invocation>),
}

impl Compilable for Expression {
    fn compile(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        match self {
            Expression::Variable(variable) => variable.compile(ctx, then),
            Expression::Constant(constant) => constant.compile(ctx, then),
            Expression::Scope(scope) => scope.compile(ctx, then),
            Expression::Declaration(declaration) => declaration.compile(ctx, then),
            Expression::ForDownTo(_) => todo!(),
            Expression::Loop(plain_loop) => plain_loop.compile(ctx, then),
            Expression::Assignment(_) => todo!(),
            Expression::GenericOperation(generic_operation) => generic_operation.compile(ctx, then),
            Expression::TypeCast(type_cast) => type_cast.compile(ctx, then),
            Expression::TypeAlias(type_alias) => type_alias.compile(ctx, then),
            Expression::Reference(_) => todo!(),
            Expression::Dereference(dereference) => dereference.compile(ctx, then),
            Expression::Invocation(invocation) => invocation.compile(ctx, then),
        }
    }

    fn compile_mut(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        match self {
            Expression::Variable(variable) => variable.compile_mut(ctx, then),
            Expression::Constant(constant) => constant.compile_mut(ctx, then),
            Expression::Scope(scope) => scope.compile_mut(ctx, then),
            Expression::Declaration(declaration) => declaration.compile_mut(ctx, then),
            Expression::ForDownTo(_) => todo!(),
            Expression::Loop(plain_loop) => plain_loop.compile_mut(ctx, then),
            Expression::Assignment(_) => todo!(),
            Expression::GenericOperation(generic_operation) => {
                generic_operation.compile_mut(ctx, then)
            }
            Expression::TypeCast(type_cast) => type_cast.compile_mut(ctx, then),
            Expression::TypeAlias(type_alias) => type_alias.compile_mut(ctx, then),
            Expression::Reference(_) => todo!(),
            Expression::Dereference(dereference) => dereference.compile_mut(ctx, then),
            Expression::Invocation(invocation) => invocation.compile_mut(ctx, then),
        }
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        match self {
            Expression::Variable(variable) => variable.compile_unit(ctx),
            Expression::Constant(constant) => constant.compile_unit(ctx),
            Expression::Scope(scope) => scope.compile_unit(ctx),
            Expression::Declaration(declaration) => declaration.compile_unit(ctx),
            Expression::ForDownTo(_) => todo!(),
            Expression::Loop(plain_loop) => plain_loop.compile_unit(ctx),
            Expression::Assignment(_) => todo!(),
            Expression::GenericOperation(generic_operation) => generic_operation.compile_unit(ctx),
            Expression::TypeCast(type_cast) => type_cast.compile_unit(ctx),
            Expression::TypeAlias(type_alias) => type_alias.compile_unit(ctx),
            Expression::Reference(_) => todo!(),
            Expression::Dereference(dereference) => dereference.compile_unit(ctx),
            Expression::Invocation(invocation) => invocation.compile_unit(ctx),
        }
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        match self {
            Expression::Variable(variable) => variable.compile_into(ctx, operand),
            Expression::Constant(constant) => constant.compile_into(ctx, operand),
            Expression::Scope(scope) => scope.compile_into(ctx, operand),
            Expression::Declaration(declaration) => declaration.compile_into(ctx, operand),
            Expression::ForDownTo(_) => todo!(),
            Expression::Loop(plain_loop) => plain_loop.compile_into(ctx, operand),
            Expression::Assignment(_) => todo!(),
            Expression::GenericOperation(generic_operation) => {
                generic_operation.compile_into(ctx, operand)
            }
            Expression::TypeCast(type_cast) => type_cast.compile_into(ctx, operand),
            Expression::TypeAlias(type_alias) => type_alias.compile_into(ctx, operand),
            Expression::Reference(_) => todo!(),
            Expression::Dereference(dereference) => dereference.compile_into(ctx, operand),
            Expression::Invocation(invocation) => invocation.compile_into(ctx, operand),
        }
    }
}

// TODO: implement recursive iter (can e.g. be used to find type of break or check use of variables)
