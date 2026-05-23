use crate::ast::compilation::context::Context;
use crate::ast::compilation::Compilable;
use crate::ast::CeriumType;
use crate::error::{CompilerResult, UnparseableConstant};
use crate::ranged::Ranged;
use crate::snippet;
use chasm_ir::{inst, Instruction, Operand};

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantValue {
    pub value: Ranged<String>,
}

impl ConstantValue {
    fn parse(&self) -> CompilerResult<(u16, CeriumType)> {
        let (range, raw_constant) = self.value.clone();

        if raw_constant.contains('.') {
            raw_constant
                .parse::<f16>()
                .map(|f| (f.to_bits(), CeriumType::F16))
                .map_err(|_| {
                    UnparseableConstant {
                        raw_constant,
                        range,
                    }
                    .into()
                })
        } else if raw_constant.starts_with('-') || raw_constant.starts_with('+') {
            raw_constant
                .parse::<i16>()
                .map(|i| (i as u16, CeriumType::I16))
                .map_err(|_| {
                    UnparseableConstant {
                        raw_constant,
                        range,
                    }
                    .into()
                })
        } else {
            raw_constant
                .parse::<u16>()
                .map(|u| (u, CeriumType::U16))
                .map_err(|_| {
                    UnparseableConstant {
                        raw_constant,
                        range,
                    }
                    .into()
                })
        }
    }
}

impl Compilable for ConstantValue {
    fn compile(
        &self,
        ctx: &mut Context,
    ) -> CompilerResult<(Vec<Instruction>, Option<(Operand, CeriumType)>)> {
        let (value, r#type) = self.parse()?;
        Ok((snippet!(), Some((Operand::Constant(value), r#type))))
    }

    fn compile_mut(
        &self,
        ctx: &mut Context,
    ) -> CompilerResult<(Vec<Instruction>, Option<(Operand, CeriumType)>)> {
        let uuid = ctx.uuid();
        let operand = Operand::Variable(uuid.clone());
        let (value, r#type) = self.parse()?;
        let body = Instruction::Alloc(uuid, 1, snippet!(inst!(Mov, op operand.clone(), val value)));
        Ok((snippet!(body), Some((operand, r#type))))
    }

    fn compile_unit(&self, ctx: &mut Context) -> CompilerResult<Vec<Instruction>> {
        Ok(snippet!())
    }

    fn compile_into(
        &self,
        ctx: &mut Context,
        operand: Operand,
    ) -> CompilerResult<(Vec<Instruction>, Option<CeriumType>)> {
        let (value, r#type) = self.parse()?;
        Ok((snippet!(inst!(Mov, op operand, val value)), Some(r#type)))
    }
}
