use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

fn main() {
		// Create re-sizable array for arguments and parse them.
		let args: Vec<String> = env::args().collect();
		println!("{:?}",args);	// This line is for debug purposes
		
		let connection = sqlite::open(":memory:").unwrap();
		let query = "CREATE TABLE IF NOT EXISTS tasks (taskid INTEGER, task TEXT);";
		connection.execute(query).unwrap();

		handle_calls(&args);
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


fn handle_calls (args: &Vec<String>)  {
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
										"list" => list_tasks(),
										"rm" => help(),
										_ => help(),
						};
				}
				// Binary with an option and extra argument was provided so handle accordingly.
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
				// Empty return help message
				_ => {
						help();
				}
		}

}

