use crate::ast::expression::Expression;
use crate::ranged::Ranged;

#[derive(Debug, Clone, PartialEq)]
pub struct GenericOperation {
    pub lhs: Ranged<Expression>,
    pub rhs: Ranged<Expression>,
    pub operator: Ranged<GenericOperator>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GenericOperator {
    Add,
    Sub,
    Mul,
    Div,
}

impl From<GenericOperation> for Expression {
    fn from(generic_operation: GenericOperation) -> Self {
        Expression::GenericOperation(Box::new(generic_operation))
    }
}
