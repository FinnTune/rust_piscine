pub fn sum(a: u8, b: u8) -> Result<u8, &'static str> {
    a.checked_add(b).ok_or("ERROR: attempt to add with overflow")
}

pub fn diff(a: i16, b: i16) -> Result<i16, &'static str> {
    a.checked_sub(b).ok_or("ERROR: attempt to subtract with overflow")
}

pub fn pro(a: i8, b: i8) -> Result<i8, &'static str> {
    a.checked_mul(b).ok_or("ERROR: attempt to multiply with overflow")
}

pub fn quo(a: f32, b: f32) -> Result<f32, &'static str> {
    if b == 0.0 {
        Err("ERROR: attempt to divide by zero")
    } else {
        Ok(a / b)
    }
}

pub fn rem(a: f32, b: f32) -> Result<f32, &'static str> {
    if b == 0.0 {
        Err("ERROR: attempt to divide by zero")
    } else {
        Ok(a % b)
    }
}