use crate::date::{self, parse_weekday};
use std::fmt::Display;
use serde::{Deserialize, Serialize};
use colored::Colorize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub text: String,
    pub created_at: String,
    pub weekday: String,
    pub repeat: bool
}

impl Task {
    pub fn build(text: String, weekday: Option<String>, repeat: bool) -> anyhow::Result<Task> {
        let weekday = match weekday {
            Some(weekday) => {
                parse_weekday(weekday)?.to_string()
            },
            None => "".to_string()
        };
        Ok(Task {
            text,
            created_at: date::get_time(),
            weekday,
            repeat
        })
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {} {} - created at {}",
            self.text.bright_green(),
            if self.repeat {
                "Repeat"
            } else {
                if !self.weekday.is_empty() {
                    "Next"
                } else {
                    "No date"
                }
            },
            self.weekday,
            self.created_at.bright_yellow()
        )
    }
}