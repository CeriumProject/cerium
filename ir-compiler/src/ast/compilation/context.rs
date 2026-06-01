use crate::ast::{CeriumType, Qualifier};
use crate::error::CompilerResult;
use chasm_ir::{Instruction, Operand};
use std::cmp::PartialEq;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum VarConfig {
    Scope,
    Var(Qualifier, CeriumType),
    Param(Qualifier, CeriumType),
    Result(Qualifier, CeriumType),
}

impl VarConfig {
    fn as_pair(&self) -> Option<(&Qualifier, &CeriumType)> {
        match self {
            VarConfig::Scope => None,
            VarConfig::Var(name, r#type)
            | VarConfig::Param(name, r#type)
            | VarConfig::Result(name, r#type) => Some((name, r#type)),
        }
    }

    fn as_pair_mut(&mut self) -> Option<(&mut Qualifier, &mut CeriumType)> {
        match self {
            VarConfig::Scope => None,
            VarConfig::Var(name, r#type)
            | VarConfig::Param(name, r#type)
            | VarConfig::Result(name, r#type) => Some((name, r#type)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    globals: HashMap<Qualifier, CeriumType>,
    attributes: HashMap<Qualifier, CeriumType>,
    vars: Vec<(VarConfig, Vec<Instruction>)>,
    counter: usize,
}

impl Context {
    pub fn new(
        globals: HashMap<Qualifier, CeriumType>,
        attributes: HashMap<Qualifier, CeriumType>,
    ) -> Context {
        Context {
            globals,
            attributes,
            vars: vec![(VarConfig::Scope, Vec::new())],
            // vars: Vars::new(),
            counter: 0,
        }
    }

    pub fn lookup(&self, name: &Qualifier) -> Option<&CeriumType> {
        self.vars
            .iter()
            .rev()
            .flat_map(|(config, _)| config.as_pair())
            .find(|(var_name, _)| **var_name == *name)
            .map(|(_, r#type)| r#type)
            .or_else(|| self.attributes.get(name))
            .or_else(|| self.globals.get(name))
    }

    pub fn change_type(&mut self, name: &Qualifier, r#type: CeriumType) -> Option<()> {
        self.vars
            .iter_mut()
            .flat_map(|(config, _)| config.as_pair_mut())
            .rfind(|(var_name, _)| **var_name == *name)
            .map(|(_, var_type)| {
                *var_type = r#type;
            })
    }

    pub fn label(&mut self) -> String {
        let result = format!("L{0}", self.counter);
        self.counter += 1;
        result
    }

    pub fn uuid(&mut self) -> String {
        format!("u_{0:X}", rand::random::<u128>())
    }

    pub fn push_var(&mut self, name: impl Into<Qualifier>, r#type: CeriumType) -> Operand {
        let name = name.into();
        self.vars
            .push((VarConfig::Var(name.clone(), r#type), Vec::new()));
        Operand::Variable(name.to_string())
    }

    pub fn push_param(&mut self, name: impl Into<Qualifier>, r#type: CeriumType) -> Operand {
        let name = name.into();
        self.vars
            .push((VarConfig::Param(name.clone(), r#type), Vec::new()));
        Operand::Variable(name.to_string())
    }

    pub fn push_result(&mut self, name: impl Into<Qualifier>, r#type: CeriumType) -> Operand {
        let name = name.into();
        self.vars
            .push((VarConfig::Result(name.clone(), r#type), Vec::new()));
        Operand::Variable(name.to_string())
    }

    // TODO: rewrite ts
    pub fn scope<T>(
        &mut self,
        body: impl FnOnce(&mut Context) -> CompilerResult<T>,
    ) -> CompilerResult<T> {
        self.vars.push((VarConfig::Scope, Vec::new()));
        let result = body(self)?;
        while let Some((cfg, code)) = self.vars.pop() {
            let end_of_scope = cfg == VarConfig::Scope;
            let code = match cfg {
                VarConfig::Scope => code,
                VarConfig::Var(var_name, r#type) => vec![Instruction::Alloc(
                    var_name.to_string(),
                    r#type.size(),
                    code,
                )],
                VarConfig::Param(param_name, r#type) => vec![Instruction::Param(
                    param_name.to_string(),
                    r#type.size(),
                    code,
                )],
                VarConfig::Result(result_name, r#type) => vec![Instruction::Result(
                    result_name.to_string(),
                    r#type.size(),
                    code,
                )],
            };
            self.vars.last_mut().unwrap().1.extend(code);
            if end_of_scope {
                break;
            }
        }
        Ok(result)
    }

    pub fn push_inst(&mut self, inst: Instruction) {
        self.vars.last_mut().unwrap().1.push(inst);
    }

    // TODO: fix and rename ts
    pub fn resolve(&mut self) -> CompilerResult<Vec<Instruction>> {
        while let Some((cfg, code)) = self.vars.pop() {
            let code = match cfg {
                VarConfig::Scope => code,
                VarConfig::Var(var_name, r#type) => vec![Instruction::Alloc(
                    var_name.to_string(),
                    r#type.size(),
                    code,
                )],
                VarConfig::Param(param_name, r#type) => vec![Instruction::Param(
                    param_name.to_string(),
                    r#type.size(),
                    code,
                )],
                VarConfig::Result(result_name, r#type) => vec![Instruction::Result(
                    result_name.to_string(),
                    r#type.size(),
                    code,
                )],
            };
            match self.vars.last_mut() {
                Some(outer) => outer.1.extend(code),
                None => return Ok(code),
            }
        }
        unreachable!()
    }
}
