#[derive(Debug, Clone, PartialEq)]
pub struct Boss {
    pub name: String,
    pub age: u8,
}

pub fn new<S: Into<String>>(name: S, age: u8) -> Boss {
    Boss {
        name: name.into(),
        age,
    }
}

impl Boss {
    pub fn new<S: Into<String>>(name: S, age: u8) -> Boss {
        Boss {
            name: name.into(),
            age,
        }
    }
}