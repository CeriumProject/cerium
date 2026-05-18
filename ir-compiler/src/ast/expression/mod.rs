use crate::ast::expression::assignment::Assignment;
use crate::ast::expression::constant_value::ConstantValue;
use crate::ast::expression::declaration::Declaration;
use crate::ast::expression::for_downto::ForDownTo;
use crate::ast::expression::generic_operation::GenericOperation;
use crate::ast::expression::plain_loop::Loop;
use crate::ast::expression::scope::Scope;
use crate::ast::expression::type_alias::TypeAlias;
use crate::ast::expression::type_cast::TypeCast;
use crate::ast::expression::variable::Variable;

pub mod assignment;
pub mod constant_value;
pub mod declaration;
pub mod for_downto;
pub mod generic_operation;
pub mod plain_loop;
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
    TypeCase(Box<TypeCast>),
    TypeAlias(Box<TypeAlias>),
}
