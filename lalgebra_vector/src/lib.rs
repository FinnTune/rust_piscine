pub trait Scalar: std::ops::Add<Output=Self> + std::ops::Mul<Output=Self> + Clone + PartialEq + std::fmt::Debug {}
impl<ScalarType> Scalar for ScalarType where ScalarType: std::ops::Add<Output=Self> + std::ops::Mul<Output=Self> + Clone + PartialEq + std::fmt::Debug {}

pub struct Vector<T: Scalar>(pub Vec<T>);

use std::ops::Add;

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }

        Some(Vector(
            self.0.iter().zip(other.0.iter()).map(|(a, b)| a.clone() + b.clone()).collect()
        ))
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new(elements: Vec<T>) -> Self {
        Vector(elements)
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        Some(
            self.0.iter().skip(1).zip(other.0.iter().skip(1))
                .fold(self.0[0].clone() * other.0[0].clone(), |acc, (a, b)| acc + a.clone() * b.clone())
        )
    }

}

impl<T: Scalar> std::fmt::Debug for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vector({:?})", self.0)
    }
}

impl<T: Scalar> Clone for Vector<T> {
    fn clone(&self) -> Self {
        Vector(self.0.clone())
    }
}

impl<T: Scalar> PartialEq for Vector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Scalar> Eq for Vector<T> {}

// Usage example
mod lalgebra_vector {
    pub use super::*;
}