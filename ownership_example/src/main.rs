fn main() {
    println!("Hello, world!");

    let mut x = String::from("A string");

    let ref1 = &mut x;
    ref1.push_str(" with more stuff"); // This is a mutable reference cannot be used again in the println! macro
    let ref2 = &x;
    let ref3 = &x;

    println!("x = {}, ref1 {}, ref2 = {}, ref3 = {} ", x, ref1, ref2, ref3);
    // Here we are using the reference to x in the println! macro which will throw an error in the compiler.
}
