mod cerium_type;
mod compilation;
mod definition;
mod expression;
mod qualifier;

pub use cerium_type::CeriumType;
pub use definition::*;
pub use expression::*;
pub use qualifier::Qualifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Script {
    pub definitions: Vec<Definition>,
}
