fn main() {
    // start from  a string literal where the value is hardcoded in our binary
    let foo = "Hello";

    // Convert ot an owned String allocating some memory on the heap
    let bar = foo.to_string();

    // Get the string literal &str of bar using the borrow operator &
    let baz = &bar;

    print!("foo: {}\nbar: {}\nbaz: {}\n", foo, bar, baz);
}
