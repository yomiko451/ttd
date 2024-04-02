use clap::Parser;
use colored::Colorize;
use ttd::{
    cli,
    storage
};

fn main() {
    let cli = cli::Cli::parse();

    if cli.path {
        match storage::path_check() {
            Ok(path) => println!("{}{}", "The path of .ttd.json file is: ".green(), path.display()),
            Err(e) => println!("Error: {}", e),
        }
    }

    match cli.command {
        Some(cli::Commands::Add { text, weekday, date, repeat }) => {
            match storage::add_task(text, weekday, date, repeat) {
                Ok(msg) => println!("{} {}", "Task added:".green(), msg),
                Err(e) => println!("Error: {}", e),
            }
        },
        Some(cli::Commands::Remove { index }) => {
            match storage::complete_task(index) {
                Ok(msg) => println!("{} {}", "Task removed:".red() ,msg),
                Err(e) => println!("Error: {}", e),
            }
        },
        Some(cli::Commands::List) => {
            if let Err(e) = storage::list_tasks() {
                println!("Error: {}", e);
            }
        },
        Some(cli::Commands::Today) => {
            if let Err(e) = storage::tasks_of_today() {
                println!("Error: {}", e);
            }
        },
        None => ()
    }
}
