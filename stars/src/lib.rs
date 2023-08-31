pub fn stars(n: u32) -> String {
    let num_stars = 2u32.pow(n);
    "*".repeat(num_stars as usize)
}