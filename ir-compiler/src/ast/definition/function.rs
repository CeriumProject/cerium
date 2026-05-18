use crate::ast::cerium_type::CeriumType;
use crate::ast::expression::Expression;
use crate::ast::qualifier::Qualifier;
use crate::ranged::Ranged;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: Ranged<Qualifier>,
    pub parameters: Vec<(Ranged<Qualifier>, Ranged<CeriumType>)>,
    pub return_type: Option<Ranged<CeriumType>>,
    pub body: Ranged<Expression>,
}
