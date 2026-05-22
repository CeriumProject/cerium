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

// TODO: implement recursive iter (can e.g. be used to find type of break or check use of variables)
