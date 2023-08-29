mod err;

use err::{ParseErr, ReadErr};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let mut file = File::open(path).map_err(|e| ReadErr { child_err: Box::new(e) })?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| ReadErr { child_err: Box::new(e) })?;

        let parsed: Result<TodoList, serde_json::Error> = serde_json::from_str(&contents);
        match parsed {
            Ok(todo_list) => {
                if todo_list.tasks.is_empty() {
                    Err(Box::new(ParseErr::Empty))
                } else {
                    Ok(todo_list)
                }
            }
            Err(e) => Err(Box::new(ParseErr::Malformed(Box::new(e)))),
        }
    }
}
