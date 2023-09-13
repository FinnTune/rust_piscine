use std::ops::{Add, Sub, Mul, Div};
pub trait Scalar: Sized + Add + Sub + Mul + Div {
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}
impl Scalar for u32 {
    type Item = u32;
    fn one() -> Self::Item {
        1
    }
    fn zero() -> Self::Item {
        0
    }
}
impl Scalar for u64 {
    type Item = u64;
    fn one() -> Self::Item {
        1
    }
    fn zero() -> Self::Item {
        0
    }
}
impl Scalar for i32 {
    type Item = i32;
    fn one() -> Self::Item {
        1
    }
    fn zero() -> Self::Item {
        0
    }
}
impl Scalar for f32 {
    type Item = f32;
    fn one() -> Self::Item {
        1.
    }
    fn zero() -> Self::Item {
        0.
    }
}
impl Scalar for f64 {
    type Item = f64;
    fn one() -> Self::Item {
        1.
    }
    fn zero() -> Self::Item {
        0.
    }
}
impl Scalar for i64 {
    type Item = i64;
    fn one() -> Self::Item {
        1
    }
    fn zero() -> Self::Item {
        0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T> + std::clone::Clone> Matrix<T> {
    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut res = Matrix::zero(n, n);
        for i in 0..n {
            res.0[i][i] = T::one();
        }
        res
    }
}