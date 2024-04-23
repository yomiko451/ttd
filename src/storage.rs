use std::{
    fs::{File, OpenOptions}, io::{self, BufRead, Seek, SeekFrom, Write}, path::PathBuf
};
use anyhow::anyhow;
use crate::{
    date,
    task::{Task, TaskType, OnceDateStatus}
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
            match &mut t.content {
                TaskType::OnceTask{ text: _, date, ref mut status } => {
                    *status = date::date_check(&date);
                },
                TaskType::WeekTask { text: _, weekday, ref mut ongoing } => {
                    *ongoing = date::weekday_check(weekday);
                },
                TaskType::MonthTask { text: _, day, ref mut ongoing } => {
                    *ongoing = date::day_check(*day);
                },
                _ => {}
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

pub fn update_bookmark(id: usize, new_progress: String) -> anyhow::Result<()> {
    let path = get_path()?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;
    let mut tasks = collect_tasks(&file)?;
    if let Some(task) = tasks.get_mut(id - 1) {
        if let TaskType::ProgressTask { text: _, progress: ref mut p } = &mut task.content {
            *p = new_progress;
        } else {
            return Err(anyhow!("error: The task is not a bookmark, please enter a valid index (e.g. 1, 2, 3, etc.)".bright_red()));
        }
    } else {
        return Err(anyhow!("error: Invalid index, please enter a valid index (e.g. 1, 2, 3, etc.)".bright_red()));
    }
    file.set_len(0)?;
    serde_json::to_writer_pretty(file, &tasks)?;
    let new_book_mark = tasks.get(id - 1).unwrap();
    println!("{} {}", "page updated!:".bright_green() , new_book_mark);
    
    Ok(())
}

pub fn parse_task(text: String, weekday: Option<String>, day: Option<usize>, date: Option<String>, progress: Option<String>) -> anyhow::Result<Task> {
    match (weekday, day, date, progress) {
        (Some(w), _, _, _) => {
            let w = date::parse_weekday(&w)?.to_string();
            let ongoing = date::weekday_check(&w);
            Ok(Task::build(TaskType::WeekTask { text, weekday: w, ongoing }))
        },
        (_, Some(d), _, _) => {
            if d <= 0 || d > 31 {
                return Err(anyhow!("error: Invalid day, please enter a valid day (e.g. 1, 2, 3, etc.)".bright_red()));
            }
            let ongoing = date::day_check(d);
            Ok(Task::build(TaskType::MonthTask { text, day: d, ongoing }))
        },
        (_, _, Some(d), _) => {
            let d = date::parse_date(&d)?.format("%Y%m%d").to_string();
            let status = date::date_check(&d);
            if let OnceDateStatus::Expired = status {
                println!("{}", "warning: The task has expired.".bright_yellow());
            }
            Ok(Task::build(TaskType::OnceTask { text, date: d, status }))
        },
        (_, _, _, Some(p)) => Ok(Task::build(TaskType::ProgressTask { text, progress: p })),
        _ => return Err(anyhow!("error: Invalid task type, please enter a valid task type (e.g. WeekTask, MonthTask, OnceTask, BookMark)".bright_red()))
    }
}

pub fn add_task(mut task: Task) -> anyhow::Result<()> {
    let path = get_path()?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;
    let mut tasks = collect_tasks(&file)?;
    task.id = tasks.len() + 1;
    let msg = format!("{}", task);
    tasks.push(task);
    serde_json::to_writer_pretty(file, &tasks)?;
    println!("{} {}", "Task added:".bright_green(), msg);

    Ok(())
}

pub fn handle_user_input() -> anyhow::Result<()> {
    println!("{}", "Enable multi-line input mode".bright_green());
    println!("{}", "Please enter tasks to be added in the format: Task content + task type + Weekday/monthday/date/page. ".bright_green());
    println!("{}", "For example: 'Do something awesome' -w Mon, 'Do something even more awesome' -o 20240402".bright_green());
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
                if input.len() == 3 {
                    let task_content = (input[0], input[2]);
                    let task_type = input[1];
                    if let Ok(task) = parse_input(task_content, task_type) {
                        add_task(task)?;
                    } else {
                        println!("{}", "error: Invalid input!".bright_red());
                        continue;
                    }
                } else {
                    println!("{}", "error: Invalid input!".bright_red());
                    continue;
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

fn parse_input(task_content: (&str, &str), task_type: &str) -> anyhow::Result<Task> {
    match task_type {
        "-w" => Ok(parse_task(task_content.0.to_owned(), Some(task_content.1.to_owned()), None, None, None)?),
        "-m" => Ok(parse_task(task_content.0.to_owned(), None, Some(task_content.1.parse::<usize>()?), None, None)?),
        "-o" => Ok(parse_task(task_content.0.to_owned(), None, None, Some(task_content.1.to_owned()), None)?),
        "-p" => Ok(parse_task(task_content.0.to_owned(), None, None, None, Some(task_content.1.to_owned()))?),
        _ => return Err(anyhow!("{}", "error: Invalid task type!".bright_red()))
    }
}

pub fn remove_task_by_id(id: Option<usize>) -> anyhow::Result<()> {
    let path = get_path()?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;
    let mut tasks = collect_tasks(&file)?;
    let index = match id {
        Some(id) => {
            if id == 0 || id > tasks.len() {
                return Err(anyhow!("{}{}", "error: Invalid task index! the task index should be between 1 and ".bright_red(), tasks.len().to_string().bright_red()));
            } else {
                id - 1
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
    let removed_task = tasks.remove(index);
    let tasks = id_reset(tasks);
    file.set_len(0)?;
    serde_json::to_writer_pretty(file, &tasks)?;
    println!("{} {}", "Task removed!:".bright_yellow() , removed_task);

    Ok(())
}

pub fn clear_tasks() -> anyhow::Result<()> {
    let path = get_path()?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;
    let tasks = collect_tasks(&file)?;
    file.set_len(0)?;
    println!("{}{}", "Task list cleared! count: ".bright_yellow(), tasks.len().to_string().bright_yellow());
    
    Ok(())
}

pub fn remove_tasks_by_filter(expired: bool, once_task: bool, month_task: bool, week_task: bool, progress_task: bool) -> anyhow::Result<()> {
    let path = get_path()?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;
    let tasks = collect_tasks(&file)?;
    if tasks.is_empty() {
        println!("{}", "warning: Task list is empty!".bright_yellow());
        return Ok(());
    }
    let origin_len = tasks.len();
    let (retained_tasks, removed_tasks): (Vec<Task>, Vec<Task>) = match (expired, once_task, month_task, week_task, progress_task) {
        (true, _, _, _, _ ) => {
            tasks.into_iter().partition(|task| {
                match &task.content {
                    TaskType::OnceTask { status: OnceDateStatus::Expired, .. } => {
                        false
                    },
                    _ => true
                }
            })
        },
        (_, true, _, _, _) => {
            tasks.into_iter().partition(|task| {
                match &task.content {
                    TaskType::OnceTask { .. } => false,
                    _ => true
                }
            })
        },
        (_, _, true, _, _) => {
            tasks.into_iter().partition(|task| {
                match &task.content {
                    TaskType::MonthTask { .. } => false,
                    _ => true
                }
            })
        },
        (_, _, _, true, _) => {
            tasks.into_iter().partition(|task| {
                match &task.content {
                    TaskType::WeekTask { .. } => false,
                    _=> true
                }
            })
        },
        (_, _, _, _, true) => {
            tasks.into_iter().partition(|task| {
                match &task.content {
                    TaskType::ProgressTask { .. } => false,
                    _ => true
                }
            })
        },
        _ => return Err(anyhow!("{}", "error: Invalid filter!".bright_red()))
    };
    file.set_len(0)?;
    let retained_tasks = id_reset(retained_tasks);
    serde_json::to_writer_pretty(file, &retained_tasks)?;
    let count = origin_len - retained_tasks.len();
    println!("{}{}", "Specified tasks removed! count: ".bright_yellow(), count.to_string().bright_yellow());
    removed_tasks.into_iter().enumerate().for_each(|(index, task)| {
        println!("{}: {}", index + 1, task)
    });

    Ok(())
}


fn id_reset(tasks: Vec<Task>) -> Vec<Task> {
    tasks.into_iter().enumerate().map(|(index, mut task)| {
        task.id = index + 1;
        task
    }).collect()
}

pub fn list_tasks_by_filter(expired: bool, once_task: bool, month_task: bool, week_task: bool, progress_task: bool) -> anyhow::Result<()> {
    let path = get_path()?;
    let file = OpenOptions::new()
        .read(true)
        .open(path)?;
    let tasks = collect_tasks(&file)?;
    if tasks.is_empty() {
        println!("{}", "warning: Task list is empty!".bright_yellow());
        return Ok(());
    }
    let selected_tasks: Vec<Task> = match (expired, once_task, month_task, week_task, progress_task) {
        (true, _, _, _, _) => {
            tasks.into_iter().filter(|task| {
                match &task.content {
                    TaskType::OnceTask { status: OnceDateStatus::Expired, .. } => {
                        true
                    },
                    _ => false
                }
            }).collect()
        },
        (_, true, _, _, _) => {
            tasks.into_iter().filter(|task| {
                match &task.content {
                    TaskType::OnceTask { .. } => true,
                    _ => false
                }
            }).collect()
        },
        (_, _, true, _, _) => {
            tasks.into_iter().filter(|task| {
                match &task.content {
                    TaskType::MonthTask { .. } => true,
                    _ => false
                }
            }).collect()
        },
        (_, _, _, true, _) => {
            tasks.into_iter().filter(|task| {
                match &task.content {
                    TaskType::WeekTask { .. } => true,
                    _ => false
                }
            }).collect()
        },
        (_, _, _, _, true) => {
            tasks.into_iter().filter(|task| {
                match &task.content {
                    TaskType::ProgressTask { .. } => true,
                    _ =>false
                }
            }).collect()
        },
        _ => tasks
    };
    if selected_tasks.is_empty() {
        println!("{}", "warning: There are no tasks with selected type!".bright_yellow());
    } else {
        selected_tasks.into_iter().enumerate().for_each(|(index, task)| {
            println!("{}: {}", index + 1, task)
        });
    }

    Ok(())
}

pub fn tasks_of_today() -> anyhow::Result<()> {
    let path = get_path()?;
    let file = OpenOptions::new()
        .read(true)
        .open(path)?;
    let tasks = collect_tasks(&file)?;
    if tasks.is_empty() {
        println!("{}", "warning: Task list is empty!".bright_yellow());
        return Ok(());
    }
    println!("{} {} {} {}.", date::get_greeting().bright_green(), "Today is".bright_green(), date::get_date().bright_green(), date::get_weekday().to_string().bright_green());
    let today_tasks: Vec<Task> = tasks.into_iter().filter(|task| {
        match &task.content {
            TaskType::OnceTask { status: OnceDateStatus::Ongoing, .. } => true,
            TaskType::WeekTask { ongoing: true, .. } => true,
            TaskType::MonthTask { ongoing: true, .. } => true,
            _ => false
        }
    }).collect();
    if !today_tasks.is_empty() {
        println!("{}", "Here is today’s todo list, have a nice day!".bright_green());
        today_tasks.into_iter().enumerate().for_each(|(index, task)| {
            println!("{}: {}", index + 1, task)
        });
    } else {
        println!("{}", "Take a break! there are no tasks today!".bright_green());
    };
    
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