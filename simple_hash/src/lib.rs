use std::collections::HashMap;

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut frequency_map: HashMap<&str, usize> = HashMap::new();

    for word in words.iter() {
        let count = frequency_map.entry(word).or_insert(0);
        *count += 1;
    }

    frequency_map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}