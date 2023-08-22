pub fn intials(names: Vec<&str>) -> Vec<String> {
    let mut initials = Vec::new();
    for name in names {
        let mut initial = String::new();
        for word in name.split_whitespace() {
            initial.push_str(&word[..1]);
        }
        initials.push(initial);
    }
    initials
}