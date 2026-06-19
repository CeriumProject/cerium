use crate::ast::compilation::context::Context;
use crate::ast::compilation::{Compilable, ConstCompilable, ConstContext};
use crate::ast::{CeriumType, Expression, Qualifier};
use crate::error::{CompilerResult, CouldNotResolveType};
use crate::ranged::Ranged;
use chasm_ir::{Instruction, Operand};

#[derive(Debug, Clone, PartialEq)]
pub struct StructInitialization {
    pub name: Ranged<Qualifier>,
    pub fields: Vec<(Ranged<Qualifier>, Ranged<Expression>)>,
}

impl Compilable for StructInitialization {
    fn compile(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        todo!()
    }

    fn compile_mut(
        &self,
        ctx: &mut Context,
        then: &mut dyn FnMut(&Operand, &CeriumType, &mut Context) -> CompilerResult<()>,
    ) -> CompilerResult<()> {
        todo!()
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<()> {
        todo!()
    }

    fn compile_into(&self, ctx: &mut Context, operand: &Operand) -> CompilerResult<CeriumType> {
        todo!()
    }
}

impl ConstCompilable for StructInitialization {
    // TODO: remove ts
    fn compile_const(&self, ctx: &mut ConstContext) -> CompilerResult<(Operand, CeriumType)> {
        let fields = ctx
            .lookup_struct(&self.name.1)
            .ok_or_else(|| CouldNotResolveType {
                name: self.name.clone(),
            })?;
        let mut words = Vec::new();
        for (idx, (field_name, field_type)) in fields.iter().enumerate() {
            let (_, value) = self
                .fields
                .iter()
                .find(|((_, name), _)| *name == *field_name)
                .unwrap(); //.ok_or_else(|| todo!("missing field"))?;
            let (op, r#type) = value.1.compile_const(unsafe {
                #[allow(mutable_transmutes)]
                std::mem::transmute::<&_, &mut _>(ctx)
            })?;
            words.push(op);
        }
        let op = ctx.push_section(vec![Instruction::RawWords(words)]);
        Ok((op, CeriumType::Struct(self.name.1.clone())))
    }
}
