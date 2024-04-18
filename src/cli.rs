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
    #[group(requires = "add_args")]
    Add{
        /// set content for the task
        #[arg(required_unless_present = "multiple")]
        text: Option<String>,
        
        /// set repeat weekday for the task
        #[arg(short, long = "week_day", group = "add_args")]
        weekday: Option<String>,

        /// set repeat monthday for the task
        #[arg(short = 'm', long = "month_day", group = "add_args")]
        day: Option<usize>,

        /// set one-time-date for the task
        #[arg(short='o', long = "once_date", group = "add_args")]
        date: Option<String>,

        /// set bookmark for a book
        #[arg(short = 'b', long = "book_page", group = "add_args")]
        page: Option<usize>,

        /// Add multiple tasks to the journal file.
        #[arg(visible_alias = "mul", long, conflicts_with = "text", group = "add_args")]
        multiple: bool
    }, 

    /// Remove one or multiple tasks from the journal file, If no arguments provided, the last task will be removed.
    #[command(visible_aliases = ["r", "rm"])]
    Remove{
        /// remove a task by id
        id: Option<usize>,

        /// remove all expired tasks
        #[arg(short, long, exclusive = true)]
        expired: bool,

        /// remove all tasks with repeat monthday
        #[arg(short, long, exclusive = true)]
        month_task: bool,

        /// remove all bookmarks
        #[arg(short, long, exclusive = true)]
        book_mark: bool,

        /// remove all tasks with one-time-date
        #[arg(short, long, exclusive = true)]
        once_task: bool,

        /// remove all tasks with repeat weekday
        #[arg(short, long, exclusive = true)]
        week_task: bool,

        /// remove all tasks
        #[arg(short, long, exclusive = true)]
        all: bool
    },

    /// List all tasks in the journal file.
    #[command(visible_aliases = ["l", "ls"])]
    List{
        /// list all expired tasks
        #[arg(short, long)]
        expired: bool,

        /// list all tasks with repeat monthday
        #[arg(short, long, exclusive = true)]
        month_task: bool,

        /// list all bookmarks
        #[arg(short, long, exclusive = true)]
        book_mark: bool,

        /// list all tasks with one-time-date
        #[arg(short, long, exclusive = true)]
        once_task: bool,

        /// list all tasks with repeat weekday
        #[arg(short, long, exclusive = true)]
        week_task: bool,
    },

    /// List tasks to be done today.
    #[command(visible_aliases = ["t", "td"])]
    Today,

    /// update page of the selected bookmark
    #[command(visible_aliases = ["u", "ud"])]
    Update{
        /// set the id of the bookmark to be updated
        #[arg(short, long)]
        id: usize,

        /// set the page of the bookmark to be updated
        #[arg(short = 'p', long)]
        new_page: usize,
    }
}