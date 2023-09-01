pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::new();

    for ch in input.chars() {
        let rotated_char = match ch {
            'a'..='z' => {
                // Normalize to 0-25 and then rotate
                let new_char = ((ch as i8 - 'a' as i8 + key).rem_euclid(26) + 'a' as i8) as u8;
                new_char as char
            }
            'A'..='Z' => {
                // Normalize to 0-25 and then rotate
                let new_char = ((ch as i8 - 'A' as i8 + key).rem_euclid(26) + 'A' as i8) as u8;
                new_char as char
            }
            _ => ch, // Keep numbers and punctuations unchanged
        };

        result.push(rotated_char);
    }

    result
}