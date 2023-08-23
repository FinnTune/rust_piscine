#[derive(Debug)]
struct Student {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
}


fn main() {
    let arthur = Student {
        id: 1,
        first_name: String::from("Arthur"),
        last_name: String::from("Dent"),
        email: String::from("and@and.com"),
        phone: String::from("555-555-5555"),
    };
    println!("{}", arthur.id);
    println!("{}", arthur.first_name);
    println!("{}", arthur.last_name);
    println!("{}", arthur.email);
    println!("{}", arthur.phone);
    println!("arthur: {:#?}", arthur); // Pound symbol is prettier printer
}

