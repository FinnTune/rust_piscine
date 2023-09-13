#[derive(Debug, PartialEq)]
pub struct Matrix(pub Vec<Vec<i32>>);


use std::ops::{Add, Sub};

impl Add for Matrix {
    type Output = Option<Matrix>;

    fn add(self, other: Matrix) -> Self::Output {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }

        let result = self.0.iter().zip(other.0.iter())
            .map(|(row_a, row_b)| {
                row_a.iter().zip(row_b.iter())
                    .map(|(&x, &y)| x + y)
                    .collect()
            })
            .collect();

        Some(Matrix(result))
    }
}

impl Sub for Matrix {
    type Output = Option<Matrix>;

    fn sub(self, other: Matrix) -> Self::Output {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }

        let result = self.0.iter().zip(other.0.iter())
            .map(|(row_a, row_b)| {
                row_a.iter().zip(row_b.iter())
                    .map(|(&x, &y)| x - y)
                    .collect()
            })
            .collect();

        Some(Matrix(result))
    }
}
