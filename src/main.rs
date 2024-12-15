use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use sqlite::Connection;

fn main() {
    // Create re-sizable array for arguments and parse them.
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args); // This line is for debug purposes

    let conn = initialize_database();

    handle_calls(&args, conn);
}

fn initialize_database() -> Connection {
    let connection = sqlite::open("tasks.db").unwrap();
    connection
        .execute("CREATE TABLE IF NOT EXISTS tasks (id INTEGER, task TEXT);")
        .unwrap();
    return connection;
}

fn add_tasks(task: &str, conn: Connection) {
    let query = format!("INSERT INTO tasks VALUES ('{}')", task);
    conn.execute(query).unwrap();

    println!("Task created");
}

fn list_tasks(conn: Connection) {
    let query = "SELECT * FROM tasks";
    conn.iterate(query, |pairs| {
        for &(name, value) in pairs.iter() {
            println!("{} - {}", name, value.unwrap());
        }
        true
    })
    .unwrap();
}

fn rm_tasks(index: &str) {
    let mut index: usize = index.parse().unwrap();
    index -= 1;

    let read = File::open("tasks").expect("Couldn't open a file");
    let buf = BufReader::new(&read);

    let mut lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Couldn't parse a line in file"))
        .collect();
    lines.remove(index);

    let mut write = File::create("tasks").expect("Couldn't write the newlist in rm");

    for task in lines {
        writeln!(write, "{task}").expect("Couldn't write to a file");
    }

    println!("Task removed")
}

// TODO: implement help function so that it know how to answer to different inputs.
fn help() {
    println!("Help in progress!");
}

fn handle_calls(args: &Vec<String>, connection: Connection) {
    // Match by argument length which determines functionality
    match args.len() {
        // Binary was only provided so print help.
        1 => {
            help();
        }
        // Binary with an option/typo was provided so respond accordingly.
        2 => {
            let command = args[1].as_str();
            let _result = match command {
                "add" => help(),
                "list" => list_tasks(connection),
                "rm" => help(),
                _ => help(),
            };
        }
        // Binary with an option and extra argument was provided so handle accordingly.
        3 => {
            let command = args[1].as_str();
            let argument = args[2].as_str();
            let _result = match command {
                "add" => add_tasks(&argument, connection),
                "list" => list_tasks(connection),
                "rm" => rm_tasks(&argument),
                _ => help(),
            };
        }
        // Empty return help message
        _ => {
            help();
        }
    }
}
