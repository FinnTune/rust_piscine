fn main() {

    // This incorrect bc x is out of scope
    // let my_reference {
    //     let mut x = String::from("A string");
    //     x
    // }
    //
    // println!("x = {}", x);

    // This is correct bc x is in scope
    let my_reference = &String::from("A string");

    println!("my_reference = {}", my_reference);
}
