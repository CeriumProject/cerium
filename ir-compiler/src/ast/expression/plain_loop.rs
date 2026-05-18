use crate::ast::expression::Expression;
use crate::ranged::Ranged;

#[derive(Debug, Clone, PartialEq)]
pub struct Loop {
    pub body: Ranged<Expression>,
}
