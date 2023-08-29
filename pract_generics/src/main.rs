struct Pair<T> {
    first: T,
    second: T,
}

// Using the {:?} debug formatter and debug trait
// fn print_elements<T: std::fmt::Debug>(elements: &[T]) {
//     //Loop through element with element and index
//     for e in elements {
//         println!("ELement: {:?}", e);
//     }
// }

// Using the {} display formatter and display trait
fn print_elements<T: std::fmt::Display>(elements: &[T]) {
    //Loop through element with element and index
    for e in elements {
        println!("ELement: {}", e);
    }
}

fn main() {
    println!("Hello, world!");
    let numbers = vec![1, 2, 3];
    let words = vec!["HELLO", "MY", "LOVE"];
    print_elements(&numbers);
    print_elements(&words);

    let integer_pair = Pair{first: 1, second:2};
    let string_pair = Pair{first: "Hello", second: "World"};

    println!("Integer pair: ({},{})", integer_pair.first, integer_pair.second);
    println!("String pair: {} {}!", string_pair.first, string_pair.second);
}
