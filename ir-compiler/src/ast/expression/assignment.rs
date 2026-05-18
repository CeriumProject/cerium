use crate::ast::expression::Expression;
use crate::ast::qualifier::Qualifier;
use crate::ranged::Ranged;

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub dest: Ranged<Qualifier>,
    pub source: Ranged<Expression>,
}
