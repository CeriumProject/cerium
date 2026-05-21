use crate::ast::expression::Expression;
use crate::ast::qualifier::Qualifier;
use crate::ranged::Ranged;

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub name: Ranged<Qualifier>,
    // pub r#type: Ranged<CeriumType>,
    pub value: Ranged<Expression>,
}
