use crate::ast::compilation::{ConstCompilable, ConstContext};
use crate::ast::optimize::OptimizeRangedExpression;
use crate::ast::{CeriumType, Expression, Qualifier};
use crate::error::{CompilerResult, MismatchedAssignmentType};
use crate::ranged::Ranged;
use chasm_ir::{Instruction, Section};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Constant {
    pub name: Ranged<Qualifier>,
    pub r#type: Ranged<CeriumType>,
    pub value: Ranged<Expression>,
}

impl Constant {
    pub fn compile(
        &self,
        globals: &HashMap<Qualifier, CeriumType>,
        structs: &HashMap<Qualifier, Vec<(Qualifier, CeriumType)>>,
    ) -> CompilerResult<Vec<Section>> {
        let mut ctx = ConstContext::new(globals.clone(), structs);
        let (op, r#type) = self.value.1.compile_const(&mut ctx)?;
        if r#type != self.r#type.1 {
            Err(MismatchedAssignmentType {
                destination: (
                    *self.name.0.start()..=*self.r#type.0.end(),
                    self.r#type.1.clone(),
                ),
                source: (self.value.0.clone(), r#type),
            })?
        }
        let mut sections = ctx.take_sections();
        sections.push(Section {
            name: String::from(format!("§c_{0}", self.name.1.to_string())),
            signature: None,
            body: vec![Instruction::Definition(self.name.1.to_string(), op)],
        });
        Ok(sections)
    }

    pub fn optimize(self) -> Self {
        Constant {
            name: self.name,
            r#type: self.r#type,
            value: self.value.optimize(),
        }
    }
}
