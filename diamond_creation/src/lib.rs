pub fn get_diamond(c: char) -> Vec<String> {
    let mut diamond: Vec<String> = Vec::new();
    let start_char = 'A';
    let end_char = c;
    let distance = (end_char as u8 - start_char as u8) as usize;

    // Upper half of the diamond including the middle line
    for i in 0..=distance {
        let mut row = String::new();
        for j in 0..=(2 * distance) {
            let char_to_insert = if j == distance - i || j == distance + i {
                (start_char as u8 + i as u8) as char
            } else {
                ' '
            };
            row.push(char_to_insert);
        }
        diamond.push(row);
    }

    // Lower half of the diamond, skipping the middle line
    for i in (0..distance).rev() {
        let mut row = String::new();
        for j in 0..=(2 * distance) {
            let char_to_insert = if j == distance - i || j == distance + i {
                (start_char as u8 + i as u8) as char
            } else {
                ' '
            };
            row.push(char_to_insert);
        }
        diamond.push(row);
    }

    diamond
}