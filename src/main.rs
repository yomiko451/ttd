use clap::Parser;
use colored::Colorize;
use ttd::{
    cli,
    storage
};

fn main() {
    let cli = cli::Cli::parse();

    storage::init().unwrap();

    if cli.path {
        match storage::path_check() {
            Ok(path) => println!("{}{}", "The path of journal file is: ".bright_green(), path.display().to_string().bright_green()),
            Err(e) => println!("{}", e),
        }
    }

    match cli.command {
        Some(cli::Commands::Add {
            text,
            weekday,
            day,
            date,
            progress,
            multiple,
        }) => {
            if multiple {
                match storage::handle_user_input() {
                    Ok(_) => {},
                    Err(e) => println!("{}", e),
                }
            } else {
                match storage::parse_task(text.unwrap(), weekday, day, date, progress) {
                    Ok(task) => {
                        if let Err(e) = storage::add_task(task) {
                            println!("{}", e);
                        }
                    },
                    Err(e) => println!("{}", e)
                }
            }
        },
        Some(cli::Commands::Remove { 
            id, 
            all, 
            expired, 
            once_task,
            month_task,
            week_task,
            progress_task
        }) => {
            match (id, all, expired, once_task, month_task, week_task, progress_task) {
                (id, false, false, false, false, false, false) => {
                    match storage::remove_task_by_id(id) {
                        Ok(_) => {}
                        Err(e) => println!("{}", e),
                    }  
                },
                (_, true, ..) => {
                    match storage::clear_tasks() {
                        Ok(_) => {},
                        Err(e) => println!("{}", e),
                    }
                },
                _ => {
                    match storage::remove_tasks_by_filter(expired, once_task, month_task, week_task, progress_task) {
                        Ok(_) => {},
                        Err(e) => println!("{}", e),
                    }
                }
            }
        },
        Some(cli::Commands::List{ expired, once_task, month_task, week_task, progress_task }) => {
            match storage::list_tasks_by_filter(expired, once_task, month_task, week_task, progress_task) {
                Ok(_) => {},
                Err(e) => println!("{}", e),
            }
        },
        Some(cli::Commands::Today) => {
            if let Err(e) = storage::tasks_of_today() {
                println!("{}", e);
            }
        },
        Some(cli::Commands::Update{ id, new_progress }) => {
            match storage::update_bookmark(id, new_progress) {
                Ok(_) => {},
                Err(e) => println!("{}", e),
            }
        },
        None => ()
    }
}
