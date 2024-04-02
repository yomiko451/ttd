use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Write tasks to the journal file.
    #[command(visible_aliases = ["a", "ad"])]
    Add{
        text: String,
        
        /// deadline
        #[arg(short, long)]
        weekday: Option<String>,

        /// Use 
        #[arg(short, long)]
        repeat: bool
     },
    /// Remove an entry from the journal file by position.
    #[command(visible_aliases = ["r", "rm"])]
    Remove{ index: usize },
    /// List all tasks in the journal file.
    #[command(visible_aliases = ["l", "ls"])]
    List,
    /// List tasks for today.
    #[command(visible_aliases = ["t", "td"])]
    Today
}