pub fn sum(a: u8, b: u8) -> u8 {
    a.checked_add(b).expect("ERROR: attempt to add with overflow")
}

pub fn diff(a: i16, b: i16) -> i16 {
    a.checked_sub(b).expect("ERROR: attempt to subtract with overflow")
}

pub fn pro(a: i8, b: i8) -> i8 {
    a.checked_mul(b).expect("ERROR: attempt to multiply with overflow")
}

pub fn quo(a: f32, b: f32) -> f32 {
    a / b
}

pub fn rem(a: f32, b: f32) -> f32 {
    a % b
}

mod scalar {
    pub use super::*;
}
