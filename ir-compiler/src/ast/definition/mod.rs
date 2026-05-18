mod function;

use crate::ast::definition::function::Function;

#[derive(Debug, Clone, PartialEq)]
pub enum Definition {
    Function(Function),
}
