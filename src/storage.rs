use std::{
    fs::{File, OpenOptions}, 
    io::{Seek, SeekFrom}, 
    path::PathBuf
};
use chrono::Local;
use anyhow::anyhow;
use crate::task::Task;

// Mon = 0
// Monday.

// Tue = 1
// Tuesday.

// Wed = 2
// Wednesday.

// Thu = 3
// Thursday.

// Fri = 4
// Friday.

// Sat = 5
// Saturday.

// Sun = 6
// Sunday.

fn get_path() -> anyhow::Result<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
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

pub fn add_task(task: Task) -> anyhow::Result<()> {
    let path = get_path()?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;
    let mut tasks = collect_tasks(&file)?;
    tasks.push(task);
    serde_json::to_writer_pretty(file, &tasks)?;

    Ok(())
}

pub fn complete_task(task_index: usize) -> anyhow::Result<()> {
    let path = get_path()?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;

    let mut tasks = collect_tasks(&file)?;

    if task_index == 0 || task_index > tasks.len() {
        return Err(anyhow!("Invalid task index"));
    }
    tasks.remove(task_index - 1);

    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;

    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_journal_file() {
        let journal_file = get_path();
        println!("Journal file: {:?}", journal_file);
    }
}