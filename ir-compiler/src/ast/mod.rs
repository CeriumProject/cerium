mod cerium_type;
mod compilation;
mod definition;
mod expression;
mod qualifier;

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
        let globals = self.parse_globals();
        self.definitions
            .iter()
            .flat_map(|definition| definition.compile(&globals))
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

    #[test]
    fn nleb() {
        let code = "fn num() -> u16 { let x = 10; x }";
        let mut parser = Parser::new(Lexer::new(code));
        let script = parser.parse().unwrap();
        let chasm = script.compile().unwrap();
        dbg!(&chasm);
    }

    #[test]
    fn bleh() {
        let code = "fn bleh(x: u16) -> u16 { let y = x * x in 10 - y }";
        let mut parser = Parser::new(Lexer::new(code));
        let script = parser.parse().unwrap();
        let chasm = script.compile().unwrap();
        dbg!(&chasm);
    }

    #[test]
    fn sqrt() {
        let code = "fn sqrt(radicand: f16) -> f16 { let approx = (radicand alias u16 / 2 + 7680) alias f16 in (radicand / approx + approx) * 0.5 }";
        let mut parser = Parser::new(Lexer::new(code));
        let script = parser.parse().unwrap();
        let chasm = script.compile().unwrap();
        dbg!(&chasm);
    }

    #[test]
    fn call() {
        let code =
            "fn a() -> i16 { b(1.0, 1) } fn b(x: f16, y: u16) -> i16 { (x + y as f16) as i16 }";
        let mut parser = Parser::new(Lexer::new(code));
        let script = parser.parse().unwrap();
        let chasm = script.compile().unwrap();
        dbg!(&chasm);
    }

    #[test]
    fn nyeyn() {
        let code = "fn a() { let x = 10; x = 5; } fn b(ptr: &i16) { *ptr = 3 as i16; }";
        let mut parser = Parser::new(Lexer::new(code));
        let script = parser.parse().unwrap();
        dbg!(&script);
        let chasm = script.compile().unwrap();
        dbg!(&chasm);
    }

    #[test]
    fn memcpy() {
        let code = "fn memcpy(dst: &u16, src: &u16, len: u16) { for len downto 0 { *dst = *src } }";
        let mut parser = Parser::new(Lexer::new(code));
        let script = parser.parse().unwrap();
        dbg!(&script);
        let chasm = script.compile().unwrap();
        dbg!(&chasm);
    }
}
