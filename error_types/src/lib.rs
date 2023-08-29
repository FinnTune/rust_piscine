pub use chrono::{Utc, NaiveDate};

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}
impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        FormError {
            form_values: (field_name, field_value),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            first_name,
            last_name,
            birth,
            birth_location,
            password,
        }
    }

    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
        if self.first_name.is_empty() {
            let err = FormError::new(
                "first_name".to_string(),
                self.first_name.clone(),
                "No user name".to_string(),
            );
            return Err(err);
        }

        let password_chars: Vec<char> = self.password.chars().collect();
        let mut has_alphabetic = false;
        let mut has_numeric = false;
        let mut has_none_alphanumeric = false;

        for ch in password_chars.iter() {
            if ch.is_ascii_alphabetic() {
                has_alphabetic = true;
            } else if ch.is_ascii_digit() {
                has_numeric = true;
            } else if !ch.is_ascii_whitespace() {
                has_none_alphanumeric = true;
            }
        }

        if password_chars.len() < 8 {
            let err = FormError::new(
                "password".to_string(),
                self.password.clone(),
                "At least 8 characters".to_string(),
            );
            return Err(err)
        } else if !(has_alphabetic && has_numeric && has_none_alphanumeric) {
            let err = FormError::new(
                "password".to_string(),
                self.password.clone(),
                "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string(),
            );
            return Err(err)
        }
        Ok(vec!["Valid first name", "Valid password"])
    }
}