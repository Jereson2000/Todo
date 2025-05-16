use clap::{Parser, Subcommand};
use sqlite::Connection;

use crate::database::{add_tasks, delete_tasks, list_tasks};

#[derive(Parser)]
#[command(name = "Todo")]
#[command(version = "0.1")]
#[command(about = "Todo cli tool for task management!", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task: String },
    List,
    Delete { id: Vec<u8> },
}

pub fn handle_arguments(cli: Cli, conn: Connection) {
    match cli.command {
        Commands::Add { task } => add_tasks(task, conn),
        Commands::Delete { id } => delete_tasks(id, conn),
        Commands::List => list_tasks(conn),
    }
}

pub fn get_arguments() -> Cli {
    return Cli::parse();
}
