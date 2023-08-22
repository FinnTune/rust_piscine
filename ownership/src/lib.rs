pub fn first_subword(mut s: String) -> String {
    if s.contains('_') {
        return s.splitn(2, '_').next().unwrap().to_string();
    }

    for (i, ch) in s.char_indices() {
        if i != 0 && ch.is_uppercase() {
            return s.splitn(2, ch).next().unwrap().to_string();
        }
    }

    s
}
