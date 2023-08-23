fn main() {
    let mut x = String::from("Hello World!");
    x.push_str(" Welcome to Rust!");
    // Must use collect otherwise mismatched types ^^^^^^^^^ expected `Vec<char>`, found `Chars<'_>`
    let y: Vec<char> = x.chars().collect();
    println!("y1 = {}", y[1]);
}
