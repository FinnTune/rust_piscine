pub fn arrange_phrase(phrase: &str) -> String {
    // Create a mutable vector to store words and their positions
    let mut words_with_positions: Vec<(&str, usize)> = Vec::new();

    // Extract words and their positions from the phrase
    for word in phrase.split_whitespace() {
        let position: usize = word.chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        words_with_positions.push((word, position));
    }

    // Sort the words based on their positions
    words_with_positions.sort_by_key(|&(_, position)| position);

    // Collect the words to form the final phrase without numbers
    words_with_positions.iter()
        .map(|&(word, _)| word.chars().filter(|c| !c.is_numeric()).collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}
