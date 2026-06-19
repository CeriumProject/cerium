use crate::ast::{CeriumType, Qualifier};
use crate::ranged::Ranged;

#[derive(Debug, Clone, PartialEq)]
pub struct Structure {
    pub name: Ranged<Qualifier>,
    pub attributes: Vec<(Ranged<Qualifier>, Ranged<CeriumType>)>,
}

impl Structure {
    pub fn signature(&self) -> (Qualifier, Vec<(Qualifier, CeriumType)>) {
        let name = self.name.1.clone();
        let attributes = self
            .attributes
            .iter()
            .map(|((_, field_name), (_, field_type))| (field_name.clone(), field_type.clone()))
            .collect();
        (name, attributes)
    }
}
