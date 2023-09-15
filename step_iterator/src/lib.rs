use std::cmp::PartialOrd;
use std::ops::{Add, AddAssign};

pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
}

impl<T: Copy + PartialOrd + Add<Output = T> + AddAssign> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator {
            current: beg,
            end: end,
            step: step,
        }
    }
}

impl<T: Copy + PartialOrd + Add<Output = T> + AddAssign> std::iter::Iterator for StepIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.end {
            let prev = self.current;
            self.current += self.step;
            Some(prev)
        } else {
            None
        }
    }
}