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
        Some(cli::Commands::Add { text, weekday, date, multiple }) => {
            if multiple {
                match storage::handle_user_input() {
                    Ok(_) => {},
                    Err(e) => println!("{}", e),
                }
            } else {
                match storage::add_task(text.unwrap(), weekday, date) {
                    Ok(_) => {},
                    Err(e) => println!("{}", e)
                }
            }
        },
        Some(cli::Commands::Remove { index, expired, all, flexible, date, weekday }) => {
            match (index, expired, all, flexible, date, weekday) {
                (index, false, false, false , false, false) => {
                    match storage::remove_task(index) {
                        Ok(_) => {}
                        Err(e) => println!("{}", e),
                    }  
                },
                (_, _, true, _, _, _) => {
                    match storage::clear_tasks() {
                        Ok(_) => {},
                        Err(e) => println!("{}", e),
                    }
                },
                _ => {
                    match storage::remove_tasks_by_filter(expired, flexible, date, weekday) {
                        Ok(_) => {},
                        Err(e) => println!("{}", e),
                    }
                }
            }
        },
        Some(cli::Commands::List{ flexible, expired, date, weekday }) => {
            match storage::list_tasks_by_filter(flexible, expired, date, weekday) {
                Ok(_) => {},
                Err(e) => println!("{}", e),
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
