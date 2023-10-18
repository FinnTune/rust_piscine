mod my_lib;

fn main() {
    my_lib::log_to_file(&format!("some nice log print"));
    my_lib::log_to_file(&format!("some nice log print2"));
}
