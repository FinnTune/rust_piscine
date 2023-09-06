use std::collections::HashMap;
use json::JsonValue;
use chrono::{DateTime, Utc};

pub fn commits_per_week(data: &JsonValue) -> HashMap<String, u32> {
    let mut commits_per_week: HashMap<String, u32> = HashMap::new();

    for commit in data.members() {
        let date_str = commit["commit"]["author"]["date"].as_str().unwrap();
        let date_time = DateTime::parse_from_rfc3339(date_str).unwrap().with_timezone(&Utc);
        let week = date_time.format("%Y-W%V").to_string();
        let count = commits_per_week.entry(week).or_insert(0);
        *count += 1;
    }

    commits_per_week
}

pub fn commits_per_author(data: &JsonValue) -> HashMap<String, u32> {
    let mut commits_per_author: HashMap<String, u32> = HashMap::new();

    for commit in data.members() {
        let author = commit["author"]["login"].as_str().unwrap().to_string();
        let count = commits_per_author.entry(author).or_insert(0);
        *count += 1;
    }

    commits_per_author
}