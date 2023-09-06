#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ThreeDVector<T> {
    pub i: T,
    pub j: T,
    pub k: T,
}

use std::ops::{Add, Sub};

// Implementing addition operation for ThreeDVector
impl<T: Add<Output = T>> Add for ThreeDVector<T> {
    type Output = ThreeDVector<T>;

    fn add(self, other: ThreeDVector<T>) -> ThreeDVector<T> {
        ThreeDVector {
            i: self.i + other.i,
            j: self.j + other.j,
            k: self.k + other.k,
        }
    }
}

// Implementing subtraction operation for ThreeDVector
impl<T: Sub<Output = T>> Sub for ThreeDVector<T> {
    type Output = ThreeDVector<T>;

    fn sub(self, other: ThreeDVector<T>) -> ThreeDVector<T> {
        ThreeDVector {
            i: self.i - other.i,
            j: self.j - other.j,
            k: self.k - other.k,
        }
    }
}
