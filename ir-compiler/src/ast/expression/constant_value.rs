use crate::ranged::Ranged;

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantValue {
    pub value: Ranged<String>,
}
