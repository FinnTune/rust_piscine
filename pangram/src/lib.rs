pub fn is_pangram(s: &str) -> bool {
    // Create a mutable HashSet to store unique alphabets
    use std::collections::HashSet;
    let mut unique_alphabets = HashSet::new();

    // Convert the string to lowercase and iterate through each character
    for c in s.to_lowercase().chars() {
        // Check if the character is an alphabet
        if c >= 'a' && c <= 'z' {
            unique_alphabets.insert(c);
        }
    }

    // Check if all 26 alphabets are present
    unique_alphabets.len() == 26
}