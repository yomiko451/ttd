use chrono::{Datelike, Local, Timelike, Weekday};
use anyhow::anyhow;

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

pub fn get_weekday() -> Weekday {
    Local::now().weekday()
}

pub fn parse_weekday(weekday: String) -> anyhow::Result<chrono::Weekday> {
    weekday.parse::<chrono::Weekday>().or(
        Err(anyhow!("Invalid weekday, please enter a valid weekday (e.g. Mon, tue, etc.)"))
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_weekday() {
        let time = get_weekday();
        println!("Today is : {}", time);
    }

    #[test]
    fn test_parse_weekday() {
        let weekday = parse_weekday("mon的".to_string());
        assert!(weekday.is_err());
    }
}