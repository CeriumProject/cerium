use crate::ast::compilation::context::Context;
use crate::ast::compilation::Compilable;
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
use crate::ast::CeriumType;
use crate::error::CompilerResult;
use chasm_ir::{Instruction, Operand};

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
    ) -> CompilerResult<(Vec<Instruction>, Option<(Operand, CeriumType)>)> {
        match self {
            Expression::Constant(constant) => constant.compile(ctx),
            Expression::Loop(plain_loop) => plain_loop.compile(ctx),
            Expression::GenericOperation(generic_operation) => generic_operation.compile(ctx),
            Expression::Scope(scope) => scope.compile(ctx),
            _ => todo!(),
        }
    }

    fn compile_mut(
        &self,
        ctx: &mut Context,
    ) -> CompilerResult<(Vec<Instruction>, Option<(Operand, CeriumType)>)> {
        match self {
            Expression::Constant(constant) => constant.compile_mut(ctx),
            Expression::Loop(plain_loop) => plain_loop.compile_mut(ctx),
            Expression::GenericOperation(generic_operation) => generic_operation.compile_mut(ctx),
            Expression::Scope(scope) => scope.compile_mut(ctx),
            _ => todo!(),
        }
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<Vec<Instruction>> {
        match self {
            Expression::Constant(constant) => constant.compile_unit(ctx),
            Expression::Loop(plain_loop) => plain_loop.compile_unit(ctx),
            Expression::GenericOperation(generic_operation) => generic_operation.compile_unit(ctx),
            Expression::Scope(scope) => scope.compile_unit(ctx),
            _ => todo!(),
        }
    }

    fn compile_into(
        &self,
        ctx: &mut Context,
        operand: Operand,
    ) -> CompilerResult<(Vec<Instruction>, Option<CeriumType>)> {
        match self {
            Expression::Constant(constant) => constant.compile_into(ctx, operand),
            Expression::Loop(plain_loop) => plain_loop.compile_into(ctx, operand),
            Expression::GenericOperation(generic_operation) => {
                generic_operation.compile_into(ctx, operand)
            }
            Expression::Scope(scope) => scope.compile_into(ctx, operand),
            _ => todo!(),
        }
    }
}

// TODO: implement recursive iter (can e.g. be used to find type of break or check use of variables)
