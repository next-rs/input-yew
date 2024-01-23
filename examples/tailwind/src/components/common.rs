use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoginUserSchema {
    email: String,
    password: String,
}

pub fn validate_email(email: String) -> bool {
    let pattern = Regex::new(r"^[^ ]+@[^ ]+\.[a-z]{2,3}$").unwrap();
    pattern.is_match(&email)
}

pub fn validate_input(field: String) -> bool {
    !&field.is_empty()
}
