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
    /// Add a task to the journal file.
    #[command(visible_aliases = ["a", "ad"])]
    Add{
        /// set content for the task
        text: Option<String>, // TODO 当没有其他参数时，这个参数必须有
        
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

    /// Remove one or multiple tasks from the journal file.
    #[command(visible_aliases = ["r", "rm"])]
    Remove{
        /// remove a task by index
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

    /// List tasks to be done today.
    #[command(visible_aliases = ["t", "td"])]
    Today
}