use crate::ast::Expression;
use crate::ranged::Ranged;

#[derive(Debug, Clone, PartialEq)]
pub struct Invocation {
    pub function: Ranged<Expression>,
    pub parameters: Vec<Ranged<Expression>>,
}
