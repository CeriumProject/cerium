use crate::ast::expression::Expression;
use crate::ast::CeriumType;
use crate::ranged::Ranged;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeCast {
    pub value: Ranged<Expression>,
    pub r#type: Ranged<CeriumType>,
}
