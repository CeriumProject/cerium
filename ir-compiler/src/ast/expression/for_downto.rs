use crate::ast::expression::Expression;
use crate::ast::qualifier::Qualifier;
use crate::ranged::Ranged;

#[derive(Debug, Clone, PartialEq)]
pub struct ForDownTo {
    pub counter: Ranged<Qualifier>,
    pub target: Ranged<Expression>,
    pub body: Ranged<Expression>,
}
