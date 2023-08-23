use std::collections::HashMap;

fn main() {
let mut apples_per_person = HashMap::new();
apples_per_person.insert("Alice", 5);
apples_per_person.insert("Bob", 3);
apples_per_person.insert("Charlie", 10);
    println!("Apples: {:?}", apples_per_person);
}
