use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Qualifier {
    pub scopes: Vec<String>,
}

impl Qualifier {
    pub fn new(scopes: Vec<String>) -> Self {
        Self { scopes }
    }

    pub fn short(scope: impl Into<String>) -> Self {
        Self {
            scopes: vec![scope.into()],
        }
    }
}

impl Display for Qualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.scopes.join("::"))
    }
}
