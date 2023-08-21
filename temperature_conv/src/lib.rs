pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    let celsius = (f - 32.0) * (5.0/9.0);
    (celsius * 1e15).round() / 1e15  // Round to 15 decimal places
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * (9.0/5.0) + 32.0
}