use crate::ast::expression::Expression;
use crate::ranged::Ranged;

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub dest: Ranged<Expression>,
    pub source: Ranged<Expression>,
}
