use std::u8;

use clap::{Parser, Subcommand};
use sqlite::Connection;

#[derive(Parser)]
#[command(name = "Todo")]
#[command(version = "0.1")]
#[command(about = "Todo list cli tool for listing tasks!", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Lists tasks
    Add { task: String },
    List,
    Done { id: u8 },
}

fn main() {
    // TODO: figure if there is a way to store the arguments as global constant as they won't be change after initialization.
    // TODO: Determine the size for the array and initialize with it.
    // Create re-sizable array for arguments and parse them.
    let cli = Cli::parse();

    // TODO: move the database to a file.
    // TODO: think how the connection should be handled
    let conn: Connection = sqlite::open("./test.db").unwrap();
    let statement =
        String::from("CREATE TABLE IF NOT EXISTS tasks(id INTEGER PRIMARY KEY, task TEXT);");
    conn.execute(statement).unwrap();

    handle_calls(cli, conn);
}

fn add_tasks(task: String, conn: Connection) {
    let statement: String = format!("INSERT INTO tasks(id, task) VALUES(1, '{task}');");
    conn.execute(statement).unwrap();
    println!("Added task: {task}");
}

// TODO: implement this better
fn list_tasks(conn: Connection) {
    let statement = String::from("SELECT * FROM tasks;");
    conn.iterate(statement, |row| {
        let collection: Vec<_> = row.iter().collect();
        println!(
            "{:?} - {:?}",
            collection[0].1.unwrap(),
            collection[1].1.unwrap()
        );
        true
    })
    .unwrap();
}

fn delete_tasks(id: u8, conn: Connection) {
    let statement = format!("delete from tasks where id={id};");
    conn.execute(statement).unwrap();
    println!("Deleted task with id: {id}")
}

fn handle_calls(cli: Cli, conn: Connection) {
    match cli.command {
        Commands::Add { task } => add_tasks(task, conn),

        Commands::Done { id } => delete_tasks(id, conn),

        Commands::List => list_tasks(conn),
    }
}
