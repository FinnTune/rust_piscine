fn main() {
    let x = 10;
    let y = 20;
    // With `move`
    let own_closure = move || {
        println!("{}, {}", x, y);  // attempts to take ownership of x and y, but because x and y are i32, they are copied instead.
    };

    own_closure();  // This works fine.
    // x and y are no longer accessible here as ownership has been moved to the closure.

    /* This also works
        let own_closure = move |x: i32, y: i32| {
        println!("{}, {}", x, y);
    };

    own_closure(x,y);
     */

    println!("{}, {}", x, y);  // error: value borrowed here after move

    // Without `move`
    let borrow_closure = || {
        println!("{}, {}", x, y);  // borrows x and y
    };

    borrow_closure();  // This works fine as x and y are still in scope.


    // Other example with non-copy type


    let name: &str = "John";

    print!("{}\n", name);  // name is still accessible here.

    consume_string(name);  // `name` is moved into `consume_string`, so it's no longer accessible in main.

    print!("{}\n", name);  // name is not accessible here as it is consumed by `consume_string`.
}

fn consume_string(s: &str) {  // `s` takes ownership of the passed value.
    print!("{}\n", s);  // prints "John"
}  // `s` goes out of scope and is dropped here, freeing the memory.