use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Use a different journal file.
    #[arg(short, long)]
    pub recycle: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Write tasks to the journal file.
    Add{ text: String },
    /// Remove an entry from the journal file by position.
    Done{ index: usize },
    /// List all tasks in the journal file.
    List
}