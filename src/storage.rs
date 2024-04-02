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
    }).ok_or(anyhow!("Could not find home directory"))
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

pub fn add_task(text: String, weekday: Option<String>, repeat: bool) -> anyhow::Result<Task> {
    let task = Task::build(text, weekday, repeat)?;
    let path = get_path()?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;
    let mut tasks = collect_tasks(&file)?;
    let msg = task.clone();
    tasks.push(task);
    serde_json::to_writer_pretty(file, &tasks)?;

    Ok(msg)
}

pub fn complete_task(task_index: usize) -> anyhow::Result<Task> {
    let path = get_path()?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;

    let mut tasks = collect_tasks(&file)?;

    if task_index == 0 || task_index > tasks.len() {
        return Err(anyhow!("Invalid task index"));
    }
    let msg = tasks.get(task_index - 1).unwrap().to_owned();
    tasks.remove(task_index - 1);

    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;

    Ok(msg)
}

pub fn list_tasks() -> anyhow::Result<()> {
    let path = get_path()?;
    let file = OpenOptions::new()
    .read(true)
    .open(path)?;

    let tasks = collect_tasks(&file)?;

    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        tasks.into_iter().enumerate().for_each(|(index, task)| {
            println!("{}: {}", index + 1, task)
        });
    }

    Ok(())
}

pub fn tasks_of_today() -> anyhow::Result<()> {
    println!("{} {} {} {}.", date::get_greeting().blue(), "Today is".blue(), date::get_date().blue(), date::get_weekday().to_string().blue());
    println!("{}", "Here is today’s to-do list, have a nice day!".blue());
    
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