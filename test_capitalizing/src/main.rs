// Test program
// Import the functions from lib.rs
use capitalizing;

fn main() {
    println!("{}", capitalizing::capitalize_first("joe is missing"));
    println!("{}", capitalizing::title_case("jill is leaving A"));
    println!("{}", capitalizing::change_case("heLLo THere"));
}