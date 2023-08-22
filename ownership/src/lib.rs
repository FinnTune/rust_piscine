pub fn first_subword(s: String) -> String {
    if s.contains('_') {
        return s.splitn(2, '_').next().unwrap().to_string();
    }

    for (i, ch) in s.char_indices() {
        if i != 0 && ch.is_uppercase() {
            return s[..i].to_string();
        }
    }

    s
}
