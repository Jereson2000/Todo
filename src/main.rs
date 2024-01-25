use std::env;
use std::fs::File;
use std::io::{Error, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    // https://doc.rust-lang.org/rust-by-example/std_misc/arg/matching.html

    match args.len() {
        1 => {
            help();
        }
        2 => {
            help();
        }
        3 => {
            let command = args[1].as_str();
            let argument = args[2].as_str();
            let result = match command {
                "add" => add_task(&argument),
                "list" => Ok(help()),
                "rm" => Ok(help()),
                _ => Ok(help()),
            };
            println!("{:?}", result);
        }
        _ => {
            help();
        }
    }
}

fn add_task(task: &str) -> Result<String, Error> {
    let mut output = File::create("tasks.txt")?;

    write!(output, "{task}\n")?;

    Ok(String::from("Successfully added a task!"))
}

// TODO: implement help function so that it know how to answer to different inputs. 
fn help() -> String {
    return String::from("Help in progress!");
}
