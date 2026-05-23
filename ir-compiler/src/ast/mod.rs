mod cerium_type;
mod compilation;
mod definition;
mod expression;
mod qualifier;

use crate::ast::compilation::context::Context;
use crate::error::CompilerResult;
pub use cerium_type::CeriumType;
pub use definition::*;
pub use expression::*;
pub use qualifier::Qualifier;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Script {
    pub definitions: Vec<Definition>,
}

impl Script {
    fn parse_globals(&self) -> HashMap<Qualifier, CeriumType> {
        self.definitions
            .iter()
            .flat_map(Definition::as_global)
            .collect()
    }

    pub fn compile(&self) -> CompilerResult<Vec<chasm_ir::Section>> {
        let context = Context::new(self.parse_globals());
        self.definitions
            .iter()
            .flat_map(|definition| definition.compile(context.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn bla() {
        let code = "fn num() -> u16 { 10 / (10 - 1) + 13 * 2 - 1 }";
        let mut parser = Parser::new(Lexer::new(code));
        let script = parser.parse().unwrap();
        let chasm = script.compile().unwrap();
        dbg!(&chasm);
    }
}
