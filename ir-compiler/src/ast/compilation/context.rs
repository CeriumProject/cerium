use crate::ast::{CeriumType, Qualifier};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Context {
    globals: HashMap<Qualifier, CeriumType>,
    vars: Vars,
    counter: usize,
}

impl Context {
    pub fn new(globals: HashMap<Qualifier, CeriumType>) -> Context {
        Context {
            globals,
            vars: Vars::new(),
            counter: 0,
        }
    }

    pub fn lookup(&self, name: &Qualifier) -> Option<&CeriumType> {
        match self.vars.lookup(name) {
            Some(r#type) => Some(r#type),
            None => self.globals.get(name),
        }
    }

    pub fn label(&mut self) -> String {
        let result = format!("L{0}", self.counter);
        self.counter += 1;
        result
    }
}

impl Deref for Context {
    type Target = Vars;

    fn deref(&self) -> &Vars {
        &self.vars
    }
}

#[derive(Debug, Clone)]
pub struct Vars {
    vars: Vec<(Qualifier, CeriumType)>,
    scopes: Vec<usize>,
}

impl Vars {
    fn new() -> Self {
        Vars {
            vars: Vec::new(),
            scopes: Vec::new(),
        }
    }

    pub fn push_var(&mut self, qualifier: Qualifier, r#type: CeriumType) {
        self.vars.push((qualifier, r#type));
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(self.vars.len());
    }

    pub fn pop_scope(&mut self) {
        if let Some(len) = self.scopes.pop() {
            self.vars.truncate(len);
        }
    }

    pub fn lookup(&self, name: &Qualifier) -> Option<&CeriumType> {
        self.vars
            .iter()
            .filter(|(var_name, _)| var_name == name)
            .map(|(_, r#type)| r#type)
            .last()
    }
}
