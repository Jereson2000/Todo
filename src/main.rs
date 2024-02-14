use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            help();
        }
        2 => {
            let command = args[1].as_str();
            let _result = match command {
                "add" => help(),
                "list" => list_tasks(),
                "rm" => help(),
                _ => help(),
            };
        }
        3 => {
            let command = args[1].as_str();
            let argument = args[2].as_str();
            let _result = match command {
                "add" => add_tasks(&argument),
                "list" => list_tasks(),
                "rm" => rm_tasks(&argument),
                _ => help(),
            };
        }
        _ => {
            help();
        }
    }
}

fn add_tasks(task: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .open("tasks")
        .expect("Couldn't open a file");

    writeln!(file, "{task}").expect("Couldn't write to a file");

    println!("Successfully added a task!");
}

fn list_tasks() {
    let file = File::open("tasks").expect("Couldn't open a file");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Couldn't parse a line in file"))
        .collect();

    let mut count = 1;
    for line in lines {
        println!("{count}. {line}");
        count += 1;
    }
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
}

// TODO: implement help function so that it know how to answer to different inputs.
fn help() {
    println!("Help in progress!");
}
