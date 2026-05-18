use crate::ast::qualifier::Qualifier;
use crate::ranged::Ranged;

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: Ranged<Qualifier>,
}
