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
    /// Add one or multiple tasks to the journal file, If no arguments provided, the task will be configured as a long-term task.
    #[command(visible_aliases = ["a", "ad"])]
    Add{
        /// set content for the task
        #[arg(required_unless_present = "multiple")]
        text: Option<String>,
        
        /// set repeat weekday for the task
        #[arg(short, long, conflicts_with = "date")]
        weekday: Option<String>,

        /// set one-time-date for the task
        #[arg(short, long)]
        date: Option<String>,

        /// Add multiple tasks to the journal file.
        #[arg(short, long, exclusive = true)]
        multiple: bool
    }, 

    /// Remove one or multiple tasks from the journal file, If no arguments provided, the last task will be removed.
    #[command(visible_aliases = ["r", "rm"])]
    Remove{
        /// remove a task by index
        index: Option<usize>,

        /// remove expired tasks
        #[arg(short, long, exclusive = true)]
        expired: bool,

        /// remove all tasks
        #[arg(short, long, exclusive = true)]
        all: bool
    },

    /// List all tasks in the journal file.
    #[command(visible_aliases = ["l", "ls"])]
    List{
        #[arg(short, long)]
        flexible: bool
    },

    /// List tasks to be done today.
    #[command(visible_aliases = ["t", "td"])]
    Today
}