pub fn num_to_ordinal(x: u32) -> String {
//Modulus operator x mod 10
if x%10 == 1 {
    format!("{}st", x)
} else if x%10 == 2 {
    format!("{}nd", x)
} else if x%10 == 3 {
    format!("{}rd", x)
} else {
    format!("{}th", x)
}
}