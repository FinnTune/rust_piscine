pub fn initials(names: Vec<&str>) -> Vec<String> {
    names.into_iter().map(|name| {
        name.split_whitespace()
            .map(|part| format!("{}.", part.chars().next().unwrap_or_default()))
            .collect::<Vec<String>>()
            .join(" ")
    }).collect()
}
