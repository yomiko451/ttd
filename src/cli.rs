use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Show the path of the journal file.
    #[arg(short, long)]
    pub path: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Write tasks to the journal file.
    #[command(visible_aliases = ["a", "ad"])]
    Add{
        /// set content for the task
        text: String,
        
        /// set repeat weekday for the task
        #[arg(short, long, conflicts_with = "date", group = "week")]
        weekday: Option<String>,

        /// set one-time-date for the task
        #[arg(short, long)]
        date: Option<String>,
     },

    /// Remove a task from the journal file by position.
    #[command(visible_aliases = ["r", "rm"])]
    Remove{
        /// remove task by index
        #[arg(required_unless_present_any = ["expired", "all"])]
        index: Option<usize>,

        /// remove expired tasks
        #[arg(short, long, conflicts_with = "index")]
        expired: bool,

        /// remove all tasks
        #[arg(short, long, exclusive = true)]
        all: bool
     },

    /// List all tasks in the journal file.
    #[command(visible_aliases = ["l", "ls"])]
    List,

    /// List tasks which are due today.
    #[command(visible_aliases = ["t", "td"])]
    Today
}