use crate::ast::{CeriumType, Qualifier};
use crate::error::CompilerResult;
use chasm_ir::{Instruction, Operand, Section};
use std::collections::HashMap;

pub trait ConstCompilable {
    // TODO: return Vec<Operand> instead of Operand -> Reference::compile_const becomes less bs
    fn compile_const(&self, ctx: &mut ConstContext) -> CompilerResult<(Operand, CeriumType)>;
}

pub struct ConstContext<'a> {
    globals: HashMap<Qualifier, CeriumType>,
    structs: &'a HashMap<Qualifier, Vec<(Qualifier, CeriumType)>>,
    sections: Vec<Section>,
}

impl<'a> ConstContext<'a> {
    pub fn new(
        globals: HashMap<Qualifier, CeriumType>,
        structs: &'a HashMap<Qualifier, Vec<(Qualifier, CeriumType)>>,
    ) -> Self {
        ConstContext {
            globals,
            structs,
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

    pub fn lookup(&self, name: &Qualifier) -> Option<&CeriumType> {
        self.globals.get(name)
    }

    pub fn lookup_struct(&self, name: &Qualifier) -> Option<&Vec<(Qualifier, CeriumType)>> {
        self.structs.get(name)
    }

    pub fn sizeof(&self, r#type: &CeriumType) -> CompilerResult<usize> {
        r#type.size(self.structs)
    }
}
