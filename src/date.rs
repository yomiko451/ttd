use chrono::{Datelike, Local, Timelike, Weekday, NaiveDate};
use anyhow::anyhow;
use crate::task::Task;

pub fn get_greeting() -> String {
    let hour = Local::now().hour();
    match hour {
        0..=11 => "Good morning!".to_string(),
        12..=16 => "Good afternoon!".to_string(),
        17..=23 => "Good evening!".to_string(),
        _ => "Hello".to_string(),
    }
}

pub fn get_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn get_date() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

pub fn parse_date(date: &str) -> anyhow::Result<NaiveDate> {
    NaiveDate::parse_from_str(date, "%Y%m%d").or(
        Err(anyhow!("Invalid date, please enter a valid date (e.g. 20240402)"))
    )
}

pub fn get_weekday() -> Weekday {
    Local::now().weekday()
}

pub fn parse_weekday(weekday: &str) -> anyhow::Result<chrono::Weekday> {
    weekday.parse::<chrono::Weekday>().or(
        Err(anyhow!("Invalid weekday, please enter a valid weekday (e.g. Mon, tue, etc.)"))
    )
}

pub fn date_check(task: &Task) -> bool {
    if !task.date.is_empty() {
        let date = parse_date(&task.date).unwrap();
        let today_date = Local::now().date_naive();
        if date == today_date {
            return true;
        }
    }
    if !task.weekday.is_empty() {
        let weekday = parse_weekday(&task.weekday).unwrap();
        let today_weekday = get_weekday();
        if weekday == today_weekday {
            return true;
        }
    }

    false
}

pub fn expired_check(task: &Task) -> bool {
    if !task.date.is_empty() {
        let date = parse_date(&task.date).unwrap();
        let today_date = Local::now().date_naive();
        if date < today_date {
            return true;
        }
    }
    
    false
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn test_get_weekday() {
        let time = get_weekday();
        println!("Today is : {}", time);
    }

    #[test]
    fn test_parse_weekday() {
        let weekday = parse_weekday("mon的");
        assert!(weekday.is_err());
    }

    #[test]
    fn test_parse_to_timestamp() {
        let timestamp_1 = parse_date("20230501").unwrap();
        let timestam0_2 = parse_date("20230503").unwrap();
        assert_eq!(timestamp_1.cmp(&timestam0_2), Ordering::Less);
    }
}