use std::{
    fs::{File, OpenOptions}, io::{self, BufRead, Seek, SeekFrom, Write}, path::PathBuf
};
use anyhow::anyhow;
use crate::{
    date::{self, parse_date, parse_weekday},
    task::Task
};
use colored::Colorize;

fn get_path() -> anyhow::Result<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".ttd.json");
        path
    }).ok_or(anyhow!("Could not find home directory.".bright_red()))
}

pub fn init() -> anyhow::Result<()> {
    let path = get_path()?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;
    let mut tasks = collect_tasks(&file)?;
    if !tasks.is_empty() {
        tasks.iter_mut().for_each(|t| {
            if date::expired_check(&t) {
                t.expired = true;
            }
        })
    }
    file.set_len(0)?;
    serde_json::to_writer_pretty(file, &tasks)?;

    Ok(())
}

pub fn path_check() -> anyhow::Result<PathBuf> {
    let path = get_path()?;
    if path.exists() {
        Ok(path)
    } else {
        Err(anyhow!("The specified file cannot be found. Please restart the program.".bright_red()))
    }
}

fn collect_tasks(mut file: &File) -> anyhow::Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;
    let tasks: Vec<Task> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?;

    Ok(tasks)
}

pub fn get_tasks_and_file() -> anyhow::Result<(File, Vec<Task>)> {
    let path = get_path()?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;
    let tasks = collect_tasks(&file)?;

    Ok((file, tasks))
}

pub fn add_task(text: String, weekday: Option<String>, date: Option<String>) -> anyhow::Result<()> {
    let mut task = Task::build(text, weekday, date)?;
    let (file, mut tasks) = get_tasks_and_file()?;
    if date::expired_check(&task) {
        task.expired = true;
        println!("{}", "warning: this task is expired!".bright_yellow())
    }
    let msg = task.clone();
    tasks.push(task);
    serde_json::to_writer_pretty(file, &tasks)?;
    println!("{} {}", "Task added:".bright_green(), msg);

    Ok(())
}

pub fn handle_user_input() -> anyhow::Result<()> {
    println!("{}", "Enable multi-line input mode".bright_green());
    println!("{}", "Please enter tasks to be added in the format: Task content + space + Weekday/year-month-day. ".bright_green());
    println!("{}", "For example: 'Do something awesome' Mon, 'Do something even more awesome' 20240402".bright_green());
    println!("{}", "Enter on an empty line if you want to exit multi-line input mode.".bright_green());
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        match io::stdin().lock().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input.is_empty() {
                    println!("{}", "Exit multi-line input mode".bright_green());
                    break;
                }
                let input = input.split(" ").collect::<Vec<&str>>();
                if input.len() == 2 {
                    let text = input[0];
                    let weekday_or_date = input[1];
                    parse_input(text, weekday_or_date)?;
                } else {
                    println!("{}", "error: Invalid input!".bright_red());
                }     
            },
            Err(error) => {
                println!("Error reading input: {}", error);
                break;
            },
        }
    }

    Ok(())
}

fn parse_input(text: &str, weekday_or_date: &str) -> anyhow::Result<()> {
    match parse_date(weekday_or_date) {
        Ok(date) => {
            add_task(text.to_string(), Option::None, Some(date.format("%Y%m%d").to_string()))?;
            return Ok(());
        },
        Err(_) => {
            match parse_weekday(weekday_or_date) {
                Ok(weekday) => {
                    add_task(text.to_string(), Some(weekday.to_string()), Option::None)?;
                },
                Err(_) => return Err(anyhow!("{}", "error: Invalid date/weekday, please enter a valid date/weekday (e.g. 20240402/Mon, FRI, tue, )".bright_red()))
            }
        }
    }

    Ok(())
}

pub fn remove_task(task_index: Option<usize>) -> anyhow::Result<Task> {
    let (file, mut tasks) = get_tasks_and_file()?;
    let index = match task_index {
        Some(index) => {
            if index == 0 || index > tasks.len() {
                return Err(anyhow!("{}{}", "error: Invalid task index! the task index should be between 1 and ".bright_red(), tasks.len().to_string().bright_red()));
            } else {
                index - 1
            }
        },
        None => {
            if tasks.is_empty() {
                return Err(anyhow!("{}", "error: There are no tasks to remove!".bright_red()));
            } else {
                tasks.len() - 1
            }
        }
    };
    let msg = tasks.get(index).unwrap().to_owned();
    tasks.remove(index);
    file.set_len(0)?;
    serde_json::to_writer_pretty(file, &tasks)?;
    println!("{} {}", "Task removed:".bright_yellow() ,msg);

    Ok(msg)
}

pub fn clear_tasks() -> anyhow::Result<()> {
    let (file, tasks) = get_tasks_and_file()?;
    file.set_len(0)?;
    println!("{}{}", "Task list cleared! count: ".bright_yellow(), tasks.len().to_string().bright_yellow());
    
    Ok(())
}

pub fn remove_tasks_by_filter(expired: bool, flexible: bool, date: bool, weekday: bool) -> anyhow::Result<()> {
    let (file, mut tasks) = get_tasks_and_file()?;
    if tasks.is_empty() {
        println!("{}", "warning: Task list is empty!".bright_yellow());
        return Ok(());
    }
    let origin_len = tasks.len();
    match (expired, flexible, date, weekday) {
        (true, _, _, _) => {
            tasks.retain(|t| !t.expired);
            file.set_len(0)?;
            serde_json::to_writer_pretty(file, &tasks)?;
            let count = origin_len - tasks.len();
            println!("{}{}", "Expired tasks removed! count: ".bright_yellow(), count.to_string().bright_yellow());
        },
        (_, true, _, _) => {
            tasks.retain(|t| !t.date.is_empty() || !t.weekday.is_empty());
            file.set_len(0)?;
            serde_json::to_writer_pretty(file, &tasks)?;
            let count = origin_len - tasks.len();
            println!("{}{}", "Flexible tasks removed! count: ".bright_yellow(),count.to_string().bright_yellow());
        },
        (_, _, true, _) => {
            tasks.retain(|t| t.date.is_empty());
            file.set_len(0)?;
            serde_json::to_writer_pretty(file, &tasks)?;
            let count = origin_len - tasks.len();
            println!("{}{}", "One-time-date tasks removed! count: ".bright_yellow(), count.to_string().bright_yellow());
        },
        _ => {
            tasks.retain(|t| t.weekday.is_empty());
            file.set_len(0)?;
            serde_json::to_writer_pretty(file, &tasks)?;
            let count = origin_len - tasks.len();
            println!("{}{}", "Repeat weekday tasks removed! count: ".bright_yellow(), count.to_string().bright_yellow());
        }
    }

    Ok(())
}

fn get_tasks() -> anyhow::Result<Vec<Task>> {
    let path = get_path()?;
    let file = OpenOptions::new()
        .read(true)
        .open(path)?;
    let tasks = collect_tasks(&file)?;

    Ok(tasks)
}

pub fn list_tasks_by_filter(flexible: bool, expired: bool, date: bool, weekday: bool) -> anyhow::Result<()> {
    let mut tasks = get_tasks()?;
    if tasks.is_empty() {
        println!("{}", "warning: Task list is empty!".bright_yellow());
        return Ok(());
    }
    match (flexible, expired, date, weekday) {
        (true, _, _, _) => {
            tasks.retain(|t| t.date.is_empty() && t.weekday.is_empty());
            if tasks.is_empty() {
                println!("{}", "warning: There are no flexible tasks!".bright_yellow());
            } else {
                tasks.into_iter().enumerate().for_each(|(index, task)| {
                    println!("{}: {}", index + 1, task)
                });
            }
        },
        (_, true, _, _) => {
            tasks.retain(|t| t.expired);
            if tasks.is_empty() {
                println!("{}", "warning: There are no expired tasks!".bright_yellow());
            } else {
                tasks.into_iter().enumerate().for_each(|(index, task)| {
                    println!("{}: {}", index + 1, task)
                });
            }
        },
        (_, _, true, _) => {
            tasks.retain(|t| !t.date.is_empty());
            if tasks.is_empty() {
                println!("{}", "warning: There are no tasks with one-time-date!".bright_yellow());
            } else {
                tasks.into_iter().enumerate().for_each(|(index, task)| {
                    println!("{}: {}", index + 1, task)
                });
            }
        },
        (_, _, _, true) => {
            tasks.retain(|t| !t.weekday.is_empty());
            if tasks.is_empty() {
                println!("{}", "warning: There are no tasks with repeat weekday!".bright_yellow());
            } else {
                tasks.into_iter().enumerate().for_each(|(index, task)| {
                    println!("{}: {}", index + 1, task)
                })
            }
        },
        _ => {
            tasks.into_iter().enumerate().for_each(|(index, task)| {
                println!("{}: {}", index + 1, task)
            });
        }
    }

    Ok(())
}

pub fn tasks_of_today() -> anyhow::Result<()> {
    println!("{} {} {} {}.", date::get_greeting().bright_green(), "Today is".bright_green(), date::get_date().bright_green(), date::get_weekday().to_string().bright_green());
    let mut tasks = get_tasks()?;
    if tasks.is_empty() {
        println!("{}", "warning: Task list is empty!".bright_yellow());
    } else {
        tasks.retain(|t| date::date_check(t));
        if !tasks.is_empty() {
            println!("{}", "Here is today’s todo list, have a nice day!".bright_green());
            tasks.into_iter().enumerate().for_each(|(index, task)| {
                println!("{}: {}", index + 1, task)
            });
        } else {
            println!("{}", "Take a break! there is no task today!".bright_green());
        };
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_journal_file() {
        let journal_file = get_path();
        println!("Journal file: {:?}", journal_file);
    }

    #[test]
    fn test_handle_user_input() {
        handle_user_input().unwrap();
    }
}