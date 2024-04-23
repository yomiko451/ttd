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
        ongoing: bool,
    },

    MonthTask {
        text: String,
        day: usize,
        ongoing: bool
    },

    OnceTask {
        text: String,
        date: String,
        status: OnceDateStatus
    },

    ProgressTask {
        text: String,
        progress: String
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OnceDateStatus {
    Expired,
    Upcoming,
    Ongoing
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
            TaskType::WeekTask { text, weekday, ongoing } => {
                let flag = if *ongoing { " Ongoing".bright_green() } else { " Upcoming".to_string().bright_yellow() };
                format!("[#{}]: {} - {}{} - repeat every week - created at {}", self.id, text.bright_blue(), weekday.bright_green(), flag, self.created_at)
            },
            TaskType::MonthTask { text, day, ongoing } => {
                let flag = if *ongoing { " Ongoing".bright_green() } else { " Upcoming".to_string().bright_yellow() };
                format!("[#{}]: {} - {}{} - repeat every month - created at {}", self.id, text.bright_blue(), day.to_string().bright_green(), flag, self.created_at)
            },
            TaskType::OnceTask { text, date, status } => {
                let flag = match status {
                    OnceDateStatus::Expired => " Expired".bright_red(),
                    OnceDateStatus::Upcoming => " Upcoming".bright_yellow(),
                    OnceDateStatus::Ongoing => " Ongoing".bright_green()
                };
                format!("[#{}]: {} - {}{} - once-time reminder - created at {}", self.id, text.bright_blue(), date.bright_green(), flag, self.created_at)
            },
            TaskType::ProgressTask { text, progress } => {
                format!("[#{}]: {} - {} - progress - created at {}", self.id, text.bright_blue(), progress.bright_green(), self.created_at)
            }
        }
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_info())
    }
}