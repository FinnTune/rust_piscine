pub fn sum(a: u8, b: u8) -> Result<u8, &'static str> {
    a.checked_add(b).ok_or("ERROR: attempt to add with overflow")
}

pub fn diff(a: i16, b: i16) -> Result<i16, &'static str> {
    a.checked_sub(b).ok_or("ERROR: attempt to subtract with overflow")
}

pub fn pro(a: i8, b: i8) -> Result<i8, &'static str> {
    a.checked_mul(b).ok_or("ERROR: attempt to multiply with overflow")
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

fn main() {
    use scalar::*;

    // sum
    match sum(234, 2) {
        Ok(result) => println!("sum : {}", result),
        Err(error) => println!("sum : {}", error),
    }
    match sum(1, 255) {
        Ok(result) => println!("sum : {}", result),
        Err(error) => println!("sum : {}", error),
    }

    // diff
    match diff(234, 2) {
        Ok(result) => println!("diff : {}", result),
        Err(error) => println!("diff : {}", error),
    }
    match diff(-32768, 32766) {
        Ok(result) => println!("diff : {}", result),
        Err(error) => println!("diff : {}", error),
    }

    // pro
    match pro(23, 2) {
        Ok(result) => println!("pro : {}", result),
        Err(error) => println!("pro : {}", error),
    }
    match pro(-128, 2) {
        Ok(result) => println!("pro : {}", result),
        Err(error) => println!("pro : {}", error),
    }

    // quo
    println!("quo : {}", quo(22.0, 2.0));
    println!("quo : {}", quo(-128.23, 2.0));

    // rem
    println!("rem : {}", rem(22.0, 2.0));
    println!("rem : {}", rem(-128.23, 2.0));
}
