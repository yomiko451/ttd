use crate::date;
use std::fmt::Display;
use serde::{Deserialize, Serialize};
use colored::Colorize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub created_at: String,
    pub content: TaskType, 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskType {
    WeekTask {
        text: String,
        weekday: String,
    },

    MonthTask {
        text: String,
        day: usize,
    },

    OnceTask {
        text: String,
        date: String,
        expired: bool
    },

    BookMark {
        text: String,
        page: usize
    }
}


impl Task {
    pub fn build(content: TaskType) -> Task {
        Task {
            id: 0,
            created_at: date::get_time(),
            content
        }
    }

    fn format_info(&self) -> String {
        match &self.content {
            TaskType::WeekTask { text, weekday } => {
                format!("[#{}]: {} - {} repeat every week - created at {}", self.id, text.bright_blue(), weekday.bright_green(), self.created_at)
            },
            TaskType::MonthTask { text, day } => {
                format!("[#{}]: {} - {} repeat every month - created at {}", self.id, text.bright_blue(), day.to_string().bright_green(), self.created_at)
            },
            TaskType::OnceTask { text, date, expired } => {
                let flag = if expired.to_owned() { 
                    " expired".bright_yellow() 
                } else { 
                    " upcoming".bright_green()
                };
                format!("[#{}]: {} - {}{} - created at {}", self.id, text.bright_blue(), date.bright_green(), flag, self.created_at)
            },
            TaskType::BookMark { text, page } => {
                format!("[#{}]: {} - page {} - created at {}", self.id, text.bright_blue(), page.to_string().bright_green(), self.created_at)
            }
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_info())
    }
}