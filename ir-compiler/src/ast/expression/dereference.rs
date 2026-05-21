use crate::ast::Expression;
use crate::ranged::Ranged;

#[derive(Debug, Clone, PartialEq)]
pub struct Dereference {
    pub inner: Ranged<Expression>,
}
