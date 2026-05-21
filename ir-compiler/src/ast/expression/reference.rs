use crate::ast::Expression;
use crate::ranged::Ranged;

#[derive(Debug, Clone, PartialEq)]
pub struct Reference {
    pub inner: Ranged<Expression>,
}
