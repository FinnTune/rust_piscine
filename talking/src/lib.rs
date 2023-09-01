pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }

    let has_alphabetic = text.chars().any(|c| c.is_alphabetic());
    let is_all_caps = has_alphabetic && text.chars().all(|c| !c.is_alphabetic() || c.is_uppercase());
    let is_question = text.trim_end().ends_with('?');

    match (is_all_caps, is_question) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "There is no need to yell, calm down!",
        (false, true) => "Sure.",
        (false, false) => "Interesting",
    }
}
