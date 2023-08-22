fn main() {
    let mut x = String::from("A string");

        let ref1 = &mut x;
        ref1.push('!');

    let ref2 = &x;
    let ref3 = &x;

    println!("x = {}, ref2 = {}, ref3 = {}", x, ref2, ref3);
}
