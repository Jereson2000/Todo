use std::env;
use std::fs::File;
use std::io::{Error, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let task = &args[1];

    match add_task(&task) {
        Ok(file) => file,
        Err(error) => panic!("Problem generating a task: {}", error),
    };
}

fn add_task(task: &str) -> Result<(), Error> {
    let mut output = File::create("tasks.txt")?;

    write!(output, "{task}")?;

    Ok(())
}
