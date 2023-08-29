pub use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use json;

mod err;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let path = Path::new(path);
        let mut file = File::open(&path).map_err(|e| err::ReadErr {
            child_err: Box::new(e),
        })?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| err::ReadErr {
            child_err: Box::new(e),
        })?;
        let json = json::parse(&contents).map_err(|e| {
            println!("{}", e);
            err::ParseErr::Malformed(Box::new(e))
        })?;
        let title = json["title"].as_str().ok_or_else(|| {
            err::ParseErr::Empty
        })?.to_string();
        let tasks = match json["tasks"].members().collect::<Vec<_>>() {
            members if members.is_empty() => {
                return Err(Box::new(err::ParseErr::Empty));
            }
            members => {
                members
                    .into_iter()
                    .map(|task| {
                        let id = task["id"].as_u32().unwrap();
                        let description = task["description"].as_str().unwrap().to_string();
                        let level = task["level"].as_u32().unwrap();
                        Task {
                            id,
                            description,
                            level,
                        }
                    })
                    .collect()
            }
        };

        Ok(TodoList { title, tasks })
    }
}