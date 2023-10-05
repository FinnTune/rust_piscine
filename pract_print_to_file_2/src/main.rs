mod my_lib;
cargo
fn main() {
    my_lib::log_to_file(&format!("some nice log print"));
    my_lib::log_to_file(&format!("some nice log print2"));
    println!("Hello, world!");
}
