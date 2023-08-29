#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub name: String,
    pub role: Role,
    pub age: u8,
}

// Standalone function for creating a new Member
pub fn new<S: Into<String>>(name: S, role: Role, age: u8) -> Member {
    Member {
        name: name.into(),
        role,
        age,
    }
}

impl Member {
    pub fn new<S: Into<String>>(name: S, role: Role, age: u8) -> Member {
        Member {
            name: name.into(),
            role,
            age,
        }
    }

    pub fn get_promotion(&mut self) {
        self.role = match self.role {
            Role::Associate => Role::Soldier,
            Role::Soldier => Role::Caporegime,
            Role::Caporegime => Role::Underboss,
            Role::Underboss => self.role.clone(),
        };
    }
}