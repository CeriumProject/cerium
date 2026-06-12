use crate::ast::{CeriumType, Qualifier};
use crate::error::CompilerResult;
use chasm_ir::{Instruction, Operand, Section};
use std::collections::HashMap;

pub trait ConstCompilable {
    fn compile_const(&self, ctx: &mut ConstContext) -> CompilerResult<(Operand, CeriumType)>;
}

pub struct ConstContext {
    globals: HashMap<Qualifier, CeriumType>,
    sections: Vec<Section>,
}

impl ConstContext {
    pub fn new(globals: HashMap<Qualifier, CeriumType>) -> Self {
        ConstContext {
            globals,
            sections: Vec::new(),
        }
    }

    pub fn push_section(&mut self, section: Vec<Instruction>) -> Operand {
        let uuid = format!("c_{0:X}", rand::random::<u128>());
        self.sections.push(Section {
            name: uuid.clone(),
            signature: None,
            body: section,
        });
        Operand::Variable(uuid)
    }

    pub fn take_sections(self) -> Vec<Section> {
        self.sections
    }
}
