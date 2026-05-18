use crate::ast::expression::Expression;
use crate::ranged::Ranged;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    pub statements: Vec<Ranged<Expression>>,
    pub result: Option<Ranged<Expression>>,
}
