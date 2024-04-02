use clap::Parser;
use colored::Colorize;
use ttd::{
    cli,
    storage
};

fn main() {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::Add { text, weekday, repeat } => {
            match storage::add_task(text, weekday, repeat) {
                Ok(msg) => println!("{} {}", "Task added:".green(), msg),
                Err(e) => println!("Error: {}", e),
            }
        },
        cli::Commands::Remove { index } => {
            match storage::complete_task(index) {
                Ok(msg) => println!("{} {}", "Task removed:".red() ,msg),
                Err(e) => println!("Error: {}", e),
            }
        },
        cli::Commands::List => {
            if let Err(e) = storage::list_tasks() {
                println!("Error: {}", e);
            }
        },
        cli::Commands::Today => {
            if let Err(e) = storage::tasks_of_today() {
                println!("Error: {}", e);
            }
        }
    }
}
