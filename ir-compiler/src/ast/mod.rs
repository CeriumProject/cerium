mod cerium_type;
mod definition;
mod expression;
mod qualifier;

use crate::ast::cerium_type::CeriumType;
use crate::ast::definition::Definition;

#[derive(Debug, Clone, PartialEq)]
pub struct Script {
    pub definitions: Vec<Definition>,
}
