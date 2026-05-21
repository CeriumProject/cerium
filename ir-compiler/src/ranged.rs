use std::ops::RangeInclusive;

pub type Ranged<T> = (RangeInclusive<usize>, T);

pub trait ToRanged: Sized {
    fn ranged(self, range: RangeInclusive<usize>) -> Ranged<Self>;
}

impl<T> ToRanged for T {
    fn ranged(self, range: RangeInclusive<usize>) -> Ranged<Self> {
        (range, self)
    }
}
