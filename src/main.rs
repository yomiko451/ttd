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
        Some(cli::Commands::Add { text, weekday, date }) => {
            match storage::add_task(text, weekday, date) {
                Ok(msg) => println!("{} {}", "Task added:".bright_green(), msg),
                Err(e) => println!("{}", e),
            }
        },
        Some(cli::Commands::Remove { index }) => {
            match storage::complete_task(index) {
                Ok(msg) => println!("{} {}", "Task removed:".bright_red() ,msg),
                Err(e) => println!("{}", e),
            }
        },
        Some(cli::Commands::List) => {
            if let Err(e) = storage::list_tasks() {
                println!("{}", e);
            }
        },
        Some(cli::Commands::Today) => {
            if let Err(e) = storage::tasks_of_today() {
                println!("{}", e);
            }
        },
        None => ()
    }
}
