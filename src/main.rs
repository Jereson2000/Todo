use dirs::home_dir;
use std::fs;
use todo::cli::{get_arguments, handle_arguments};
use todo::database::create_connection;

fn main() {
    let home_dir: String = String::from(home_dir().unwrap().to_str().unwrap());
    let path = format!("{home_dir}/.local/share/todo");
    create_directory(&path);

    let cli = get_arguments();
    let conn = create_connection(path);
    handle_arguments(cli, conn);
}

fn create_directory(path: &String) {
    match fs::exists(path) {
        Ok(exists) => {
            if !exists {
                println!("Creating directory {}", path);
                fs::create_dir(path).unwrap()
            }
        }
        Err(_) => println!("Can't open {}!", path),
    }
}
