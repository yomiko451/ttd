use clap::Parser;
use ttd::{
    cli,
    task::Task,
    storage::{
        add_task,
        complete_task,
        list_tasks
    }
};

fn main() {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Add { text } => {
            if cli.recycle {
                println!("Adding task: {}", text);
            } else {
                match add_task(Task::new(text.to_owned())) {
                    Ok(_) => println!("Added task: {}", text),
                    Err(e) => println!("Error: {}", e),
                }
            }
            if let Err(e) = list_tasks() {
                println!("Error: {}", e);
            }
        },
        cli::Commands::Done { index } => {
            match complete_task(index.to_owned()) {
                Ok(_) => println!("Completed task: {}", index),
                Err(e) => println!("Error: {}", e),
            }
            if let Err(e) = list_tasks() {
                println!("Error: {}", e);
            }
        },
        cli::Commands::List => {
            if let Err(e) = list_tasks() {
                println!("Error: {}", e);
            }
        }
    }
}
