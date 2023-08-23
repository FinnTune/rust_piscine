pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut chars_s1: Vec<char> = s1.chars().collect();
    let mut chars_s2: Vec<char> = s2.chars().collect();

    chars_s1.sort();
    chars_s2.sort();

    chars_s1 == chars_s2
}