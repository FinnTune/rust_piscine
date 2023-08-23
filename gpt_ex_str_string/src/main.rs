// Example 1: String slice

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }
//
// fn main() {
//     let s = String::from("hello world");
//     let word = first_word(&s);
//     println!("First word: {}", word);  // Outputs "hello"
// }

// Example 2: String slice
fn swap_first_two_chars(s: &mut String) {
    let chars: Vec<_> = s.chars().collect();
    if chars.len() > 1 {
        let mut swapped = chars[1].to_string();
        swapped.push(chars[0]);
        swapped.extend(&chars[2..]);
        *s = swapped;
    }
}

fn main() {
    let mut s = String::from("𠀀melon");  // The character "𠀀" followed by the word "melon"

    println!("Before: {}", s);  // Outputs: 𠀀melon

    swap_first_two_chars(&mut s);
    println!("After: {}", s);   // Outputs: melon𠀀
}
