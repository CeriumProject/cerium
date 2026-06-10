#![feature(f16)]
#![feature(box_patterns)]

use crate::error::CompilerResult;
use crate::lexer::Lexer;
use crate::parser::Parser;
use chasm_ir::Section;

mod ast;
pub mod error;
mod lexer;
mod parser;
pub mod ranged;
mod token;

pub fn compile(src: &str) -> CompilerResult<Vec<Section>> {
    Parser::new(Lexer::new(src)).parse()?.compile()
}
