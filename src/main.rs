use clap::{Parser, Subcommand};
use sqlite::Connection;
use std::env;
use std::fs;

#[derive(Parser)]
#[command(name = "Todo")]
#[command(version = "0.1")]
#[command(about = "Todo cli tool for task management!", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task: String },
    List,
    Delete { id: Vec<u8> },
}

fn main() {
    let home_dir: String = String::from(env::home_dir().unwrap().to_str().unwrap());
    println!("{}", home_dir);
    let app_path = format!("{home_dir}/.local/share/todo");
    create_directory(&app_path);

    let cli = Cli::parse();
    let data_path = format!("{app_path}/tasks.db");
    let conn: Connection = sqlite::open(data_path).unwrap();
    let statement = String::from(
        "CREATE TABLE IF NOT EXISTS tasks(
            id INTEGER PRIMARY KEY,
            task TEXT);",
    );
    conn.execute(statement).unwrap();

    handle_calls(cli, conn);
}

fn add_tasks(task: String, conn: Connection) {
    let statement: String = format!("INSERT INTO tasks(task) VALUES('{task}');");
    conn.execute(statement).unwrap();
    println!("Added task: {task}");
}

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

fn delete_tasks(id: Vec<u8>, conn: Connection) {
    for i in id {
        let statement = format!("DELETE FROM tasks WHERE id=={i};");
        conn.execute(statement).unwrap();
        println!("Deleted task with id: {i}")
    }
}

fn handle_calls(cli: Cli, conn: Connection) {
    match cli.command {
        Commands::Add { task } => add_tasks(task, conn),

        Commands::Delete { id } => delete_tasks(id, conn),

        Commands::List => list_tasks(conn),
    }
}

fn create_directory(path: &String) {
    match fs::exists(path) {
        Ok(exists) => {
            if !exists {
                fs::create_dir(path).unwrap()
            }
        }
        Err(_) => println!("Insufficient permissions perhaps!"),
    }
}
