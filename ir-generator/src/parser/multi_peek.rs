use std::collections::VecDeque;

pub struct MultiPeek<I: Iterator> {
    iterator: I,
    peek: VecDeque<I::Item>,
}

impl<I: Iterator> MultiPeek<I> {
    pub fn new(iterator: I) -> Self {
        MultiPeek {
            iterator,
            peek: VecDeque::new(),
        }
    }

    pub fn next_if(&mut self, predicate: impl FnOnce(&I::Item) -> bool) -> Option<I::Item> {
        if self.peek().is_some_and(predicate) {
            self.peek.pop_front()
        } else {
            None
        }
    }

    pub fn peek(&mut self) -> Option<&I::Item> {
        self.peek_nth(0)
    }

    pub fn peek_nth(&mut self, n: usize) -> Option<&I::Item> {
        while self.peek.get(n).is_none() {
            self.peek.push_back(self.iterator.next()?);
        }

        self.peek.get(n)
    }
}

impl<I: Iterator> Iterator for MultiPeek<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.peek.pop_front().or_else(|| self.iterator.next())
    }
}

pub trait IntoMultiPeek: Iterator + Sized {
    fn multi_peek(self) -> MultiPeek<Self>;
}

impl<I: Iterator> IntoMultiPeek for I {
    fn multi_peek(mut self) -> MultiPeek<Self> {
        MultiPeek::new(self)
    }
}
