use std::env;
use std::fs::File;
use std::io::{Error, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = "Mene töihin";

    // TODO: Matching by arguments length
    // https://doc.rust-lang.org/rust-by-example/std_misc/arg/matching.html

    match args.len() {
        _ => {
            help();
        }
    }

    let response = add_task(&task);

    print!("{:?}", response)
}

fn add_task(task: &str) -> Result<String, Error> {
    let mut output = File::create("tasks.txt")?;

    write!(output, "{task}\n")?;

    Ok(String::from("Successfully added task!\n"))
}

fn help() {
    println!("Help in progress!")
}