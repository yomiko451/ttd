use chrono::Local;
use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub text: String,
    pub created_at: String,
    pub recycle: bool,
}

impl Task {
    pub fn new(text: String) -> Task {
        Task {
            text,
            created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            recycle: false,
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {}",
            self.text,
            self.created_at
        )
    }
}