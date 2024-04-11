use crate::date;
use std::fmt::Display;
use serde::{Deserialize, Serialize};
use colored::Colorize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub text: String,
    pub created_at: String,
    pub weekday: String,
    pub date: String,
    pub expired: bool
}

pub enum TaskStatus {
    Expired,
    Weekday,
    Flexible,
    Date
}

impl Task {
    pub fn build(text: String, weekday: Option<String>, date: Option<String>) -> anyhow::Result<Task> {
        let weekday = match weekday {
            Some(weekday) => {
                date::parse_weekday(&weekday)?.to_string()
            },
            None => "".to_string()
        };
        let date = match date {
            Some(date) => {
                date::parse_date(&date)?.format("%Y%m%d").to_string()
            },
            None => "".to_string()
        };
        Ok(Task {
            id: 0,
            text,
            created_at: date::get_time(),
            weekday,
            date,
            expired: false
        })
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} - {}{}{} - created at {}",
            format!("[task #{}]", self.id).bright_green(),
            self.text.bright_cyan(),
            self.weekday.bright_green(),
            self.date.bright_green(),
            if self.expired {
                " Expired".bright_yellow()
            } else if !self.date.is_empty() {
                " Ok".bright_green()
            } else if !self.weekday.is_empty() {
                " Repeat".bright_green()
            } else {
                "Flexible".to_string().bright_green()
            },
            self.created_at
        )
    }
}