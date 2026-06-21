use crate::ast::CeriumType;
use crate::ast::compilation::context::Context;
use crate::ast::compilation::{Compilable, ConstCompilable, ConstContext};
use crate::ast::dereference::Dereference;
pub use crate::ast::expression::array::Array;
pub use crate::ast::expression::array_indexation::ArrayIndexation;
pub use crate::ast::expression::assignment::Assignment;
use crate::ast::expression::compiler_macro::CompilerMacro;
pub use crate::ast::expression::constant_value::ConstantValue;
pub use crate::ast::expression::declaration::Declaration;
pub use crate::ast::expression::for_downto::ForDownTo;
pub use crate::ast::expression::generic_operation::GenericOperation;
use crate::ast::expression::optimize::OptimizeExpression;
pub use crate::ast::expression::plain_loop::Loop;
pub use crate::ast::expression::scope::Scope;
pub use crate::ast::expression::sizeof::Sizeof;
pub use crate::ast::expression::type_alias::TypeAlias;
pub use crate::ast::expression::type_cast::TypeCast;
pub use crate::ast::expression::unary_operation::UnaryOperator;
pub use crate::ast::expression::variable::Variable;
use crate::ast::field_access::FieldAccess;
pub use crate::ast::invocation::Invocation;
pub use crate::ast::reference::Reference;
use crate::ast::struct_initialization::StructInitialization;
use crate::ast::unary_operation::UnaryOperation;
use crate::error::CompilerResult;
use chasm_ir::Operand;

pub mod array;
pub mod array_indexation;
pub mod assignment;
pub mod compiler_macro;
pub mod constant_value;
pub mod declaration;
pub mod dereference;
pub mod field_access;
pub mod for_downto;
pub mod generic_operation;
pub mod invocation;
mod iter;
pub mod optimize;
pub mod plain_loop;
pub mod reference;
pub mod scope;
pub mod sizeof;
pub mod struct_initialization;
pub mod type_alias;
pub mod type_cast;
pub mod unary_operation;
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
    CompilerMacro(Box<CompilerMacro>),
    ArrayIndexation(Box<ArrayIndexation>),
    Array(Box<Array>),
    Sizeof(Box<Sizeof>),
    FieldAccess(Box<FieldAccess>),
    StructInitialization(Box<StructInitialization>),
    UnaryOperation(Box<UnaryOperation>),
}

// TODO: impl Deref Expression -> &dyn Compilable instead of ts
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
            Expression::ForDownTo(for_downto) => for_downto.compile(ctx, then),
            Expression::Loop(plain_loop) => plain_loop.compile(ctx, then),
            Expression::Assignment(assignment) => assignment.compile(ctx, then),
            Expression::GenericOperation(generic_operation) => generic_operation.compile(ctx, then),
            Expression::TypeCast(type_cast) => type_cast.compile(ctx, then),
            Expression::TypeAlias(type_alias) => type_alias.compile(ctx, then),
            Expression::Reference(reference) => reference.compile(ctx, then),
            Expression::Dereference(dereference) => dereference.compile(ctx, then),
            Expression::Invocation(invocation) => invocation.compile(ctx, then),
            Expression::CompilerMacro(compiler_macro) => compiler_macro.compile(ctx, then),
            Expression::ArrayIndexation(array_indexation) => array_indexation.compile(ctx, then),
            Expression::Array(array) => array.compile(ctx, then),
            Expression::Sizeof(sizeof) => sizeof.compile(ctx, then),
            Expression::FieldAccess(field_access) => field_access.compile(ctx, then),
            Expression::StructInitialization(struct_initialization) => {
                struct_initialization.compile(ctx, then)
            }
            Expression::UnaryOperation(unary_operation) => unary_operation.compile(ctx, then),
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
            Expression::ForDownTo(for_downto) => for_downto.compile_mut(ctx, then),
            Expression::Loop(plain_loop) => plain_loop.compile_mut(ctx, then),
            Expression::Assignment(assignment) => assignment.compile_mut(ctx, then),
            Expression::GenericOperation(generic_operation) => {
                generic_operation.compile_mut(ctx, then)
            }
            Expression::TypeCast(type_cast) => type_cast.compile_mut(ctx, then),
            Expression::TypeAlias(type_alias) => type_alias.compile_mut(ctx, then),
            Expression::Reference(reference) => reference.compile_mut(ctx, then),
            Expression::Dereference(dereference) => dereference.compile_mut(ctx, then),
            Expression::Invocation(invocation) => invocation.compile_mut(ctx, then),
            Expression::CompilerMacro(compiler_macro) => compiler_macro.compile_mut(ctx, then),
            Expression::ArrayIndexation(array_indexation) => {
                array_indexation.compile_mut(ctx, then)
            }
            Expression::Array(array) => array.compile_mut(ctx, then),
            Expression::Sizeof(sizeof) => sizeof.compile_mut(ctx, then),
            Expression::FieldAccess(field_access) => field_access.compile_mut(ctx, then),
            Expression::StructInitialization(struct_initialization) => {
                struct_initialization.compile_mut(ctx, then)
            }
            Expression::UnaryOperation(unary_operation) => unary_operation.compile_mut(ctx, then),
        }
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        match self {
            Expression::Variable(variable) => variable.compile_unit(ctx),
            Expression::Constant(constant) => constant.compile_unit(ctx),
            Expression::Scope(scope) => scope.compile_unit(ctx),
            Expression::Declaration(declaration) => declaration.compile_unit(ctx),
            Expression::ForDownTo(for_downto) => for_downto.compile_unit(ctx),
            Expression::Loop(plain_loop) => plain_loop.compile_unit(ctx),
            Expression::Assignment(assignment) => assignment.compile_unit(ctx),
            Expression::GenericOperation(generic_operation) => generic_operation.compile_unit(ctx),
            Expression::TypeCast(type_cast) => type_cast.compile_unit(ctx),
            Expression::TypeAlias(type_alias) => type_alias.compile_unit(ctx),
            Expression::Reference(reference) => reference.compile_unit(ctx),
            Expression::Dereference(dereference) => dereference.compile_unit(ctx),
            Expression::Invocation(invocation) => invocation.compile_unit(ctx),
            Expression::CompilerMacro(compiler_macro) => compiler_macro.compile_unit(ctx),
            Expression::ArrayIndexation(array_indexation) => array_indexation.compile_unit(ctx),
            Expression::Array(array) => array.compile_unit(ctx),
            Expression::Sizeof(sizeof) => sizeof.compile_unit(ctx),
            Expression::FieldAccess(field_access) => field_access.compile_unit(ctx),
            Expression::StructInitialization(struct_initialization) => {
                struct_initialization.compile_unit(ctx)
            }
            Expression::UnaryOperation(unary_operation) => unary_operation.compile_unit(ctx),
        }
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        match self {
            Expression::Variable(variable) => variable.compile_into(ctx, operand),
            Expression::Constant(constant) => constant.compile_into(ctx, operand),
            Expression::Scope(scope) => scope.compile_into(ctx, operand),
            Expression::Declaration(declaration) => declaration.compile_into(ctx, operand),
            Expression::ForDownTo(for_downto) => for_downto.compile_into(ctx, operand),
            Expression::Loop(plain_loop) => plain_loop.compile_into(ctx, operand),
            Expression::Assignment(assignment) => assignment.compile_into(ctx, operand),
            Expression::GenericOperation(generic_operation) => {
                generic_operation.compile_into(ctx, operand)
            }
            Expression::TypeCast(type_cast) => type_cast.compile_into(ctx, operand),
            Expression::TypeAlias(type_alias) => type_alias.compile_into(ctx, operand),
            Expression::Reference(reference) => reference.compile_into(ctx, operand),
            Expression::Dereference(dereference) => dereference.compile_into(ctx, operand),
            Expression::Invocation(invocation) => invocation.compile_into(ctx, operand),
            Expression::CompilerMacro(compiler_macro) => compiler_macro.compile_into(ctx, operand),
            Expression::ArrayIndexation(array_indexation) => {
                array_indexation.compile_into(ctx, operand)
            }
            Expression::Array(array) => array.compile_into(ctx, operand),
            Expression::Sizeof(sizeof) => sizeof.compile_into(ctx, operand),
            Expression::FieldAccess(field_access) => field_access.compile_into(ctx, operand),
            Expression::StructInitialization(struct_initialization) => {
                struct_initialization.compile_into(ctx, operand)
            }
            Expression::UnaryOperation(unary_operation) => {
                unary_operation.compile_into(ctx, operand)
            }
        }
    }
}

impl ConstCompilable for Expression {
    fn compile_const(&self, ctx: &mut ConstContext) -> CompilerResult<(Operand, CeriumType)> {
        match self {
            Expression::Constant(constant) => constant.compile_const(ctx),
            Expression::Reference(reference) => reference.compile_const(ctx),
            Expression::TypeAlias(type_alias) => type_alias.compile_const(ctx),
            Expression::Variable(variable) => variable.compile_const(ctx),
            Expression::Sizeof(sizeof) => sizeof.compile_const(ctx),
            _ => todo!("throw error"),
        }
    }
}

impl OptimizeExpression for Expression {
    fn optimize(self) -> Expression {
        match self {
            Expression::Variable(expression) => expression.optimize(),
            Expression::Constant(expression) => expression.optimize(),
            Expression::Scope(expression) => expression.optimize(),
            Expression::Declaration(expression) => expression.optimize(),
            Expression::ForDownTo(expression) => expression.optimize(),
            Expression::Loop(expression) => expression.optimize(),
            Expression::Assignment(expression) => expression.optimize(),
            Expression::GenericOperation(expression) => expression.optimize(),
            Expression::TypeCast(expression) => expression.optimize(),
            Expression::TypeAlias(expression) => expression.optimize(),
            Expression::Reference(expression) => expression.optimize(),
            Expression::Dereference(expression) => expression.optimize(),
            Expression::Invocation(expression) => expression.optimize(),
            Expression::CompilerMacro(expression) => expression.optimize(),
            Expression::ArrayIndexation(expression) => expression.optimize(),
            Expression::Array(expression) => expression.optimize(),
            Expression::Sizeof(expression) => expression.optimize(),
            Expression::FieldAccess(expression) => expression.optimize(),
            Expression::StructInitialization(expression) => expression.optimize(),
            Expression::UnaryOperation(expression) => expression.optimize(),
        }
    }
}

// TODO: implement recursive iter (can e.g. be used to find type of break or check use of variables)
