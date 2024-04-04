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
        Some(cli::Commands::Remove { index, expired, all }) => {
            if expired {
                match storage::remove_expired_tasks() {
                    Ok(_) => {},
                    Err(e) => println!("{}", e),
                }
            } else if all {
                match storage::clear_tasks() {
                    Ok(_) => {},
                   Err(e) => println!("{}", e),
                }
            } else {
                match storage::remove_task(index) {
                    Ok(_) => {}
                    Err(e) => println!("{}", e),
                }  
            }
        },
        Some(cli::Commands::List{ flexible }) => {
            match storage::list_tasks(flexible) {
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
