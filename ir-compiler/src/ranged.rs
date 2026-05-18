use std::ops::{Deref, RangeInclusive};

#[derive(Clone, Debug, PartialEq)]
pub struct Ranged<T> {
    pub range: RangeInclusive<usize>,
    pub value: T,
}

impl<T> Ranged<T> {
    pub fn new(range: RangeInclusive<usize>, value: T) -> Self {
        Self { range, value }
    }
}

impl<T> Deref for Ranged<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub trait ToRanged: Sized {
    fn ranged(self, range: RangeInclusive<usize>) -> Ranged<Self>;
}

impl<T> ToRanged for T {
    fn ranged(self, range: RangeInclusive<usize>) -> Ranged<Self> {
        Ranged { range, value: self }
    }
}
