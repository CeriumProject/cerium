use crate::ast::expression::Expression;
use crate::ranged::Ranged;

#[derive(Debug, Clone, PartialEq)]
pub struct GenericOperation {
    pub operands: Vec<Ranged<Expression>>,
    pub operator: Ranged<GenericOperator>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GenericOperator {
    Add,
    Sub,
    Mul,
    Div,
}
