use std::{
    fs::{File, OpenOptions}, 
    io::{Seek, SeekFrom}, 
    path::PathBuf
};
use anyhow::anyhow;
use crate::{
    date,
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

pub fn add_task(text: String, weekday: Option<String>, date: Option<String>) -> anyhow::Result<Task> {
    let mut task = Task::build(text, weekday, date)?;
    let (file, mut tasks) = get_tasks_and_file()?;
    if date::expired_check(&task) {
        task.expired = true;
        println!("{}", "warning: this task is expired!".bright_yellow())
    }
    let msg = task.clone();
    tasks.push(task);
    serde_json::to_writer_pretty(file, &tasks)?;

    Ok(msg)
}

pub fn remove_task(task_index: Option<usize>) -> anyhow::Result<Task> {
    let task_index = task_index.ok_or(anyhow!("{}", "error: Invalid task index!".bright_red()))?;
    let (file, mut tasks) = get_tasks_and_file()?;
    if task_index == 0 || task_index > tasks.len() {
        return Err(anyhow!("{}{}", "error: Invalid task index! the task index should be between 1 and ".bright_red(), tasks.len().to_string().bright_red()));
    }
    let msg = tasks.get(task_index - 1).unwrap().to_owned();
    tasks.remove(task_index - 1);
    file.set_len(0)?;
    serde_json::to_writer_pretty(file, &tasks)?;

    Ok(msg)
}

pub fn clear_tasks() -> anyhow::Result<usize> {
    let (file, tasks) = get_tasks_and_file()?;
    file.set_len(0)?;

    Ok(tasks.len())
}

pub fn remove_expired_tasks() -> anyhow::Result<usize> {
    let (file, mut tasks) = get_tasks_and_file()?;
    let origin_len = tasks.len();
    tasks.retain(|t| !t.expired);
    file.set_len(0)?;
    serde_json::to_writer_pretty(file, &tasks)?;

    Ok(origin_len - tasks.len())
}

fn get_tasks() -> anyhow::Result<Vec<Task>> {
    let path = get_path()?;
    let file = OpenOptions::new()
        .read(true)
        .open(path)?;
    let tasks = collect_tasks(&file)?;

    Ok(tasks)
}

pub fn list_tasks() -> anyhow::Result<()> {
    let tasks = get_tasks()?;
    if tasks.is_empty() {
        println!("{}", "warning: Task list is empty!".bright_yellow());
    } else {
        tasks.into_iter().enumerate().for_each(|(index, task)| {
            println!("{}: {}", index + 1, task)
        });
    }

    Ok(())
}

pub fn tasks_of_today() -> anyhow::Result<()> {
    println!("{} {} {} {}.", date::get_greeting().bright_green(), "Today is".bright_green(), date::get_date().bright_green(), date::get_weekday().to_string().bright_green());
    let tasks = get_tasks()?;
    if tasks.is_empty() {
        println!("{}", "warning: Task list is empty!".bright_yellow());
    } else {
        let tasks = tasks.into_iter().filter(|t| date::date_check(t)).collect::<Vec<Task>>();
        if !tasks.is_empty() {
            println!("{}", "Here is today’s to-do list, have a nice day!".bright_green());
            let mut index = 1;
            for task in tasks {
                println!("{}: {}", index, task);
                index += 1;
            }
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
}