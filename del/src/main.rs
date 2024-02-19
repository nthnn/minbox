extern crate colored;

use std::env;
use std::fs::metadata;
use std::process::exit;
use delete;
use colored::Colorize;

fn delete_folder(path: String) {
    match delete::delete_folder(path.as_str()) {
        Ok(_)=> println!("Folder '{}' successfully {}.", path.italic(), "deleted".red().bold()),
        Err(_)=> println!("Error {} folder '{}'.", "deleting".red().bold(), path.italic())
    }
}

fn delete_file(path: String) {
    match delete::delete_file(path.as_str()) {
        Ok(_)=> println!("File '{}' successfully {}.", path.italic(), "deleted".red().bold()),
        Err(_)=> println!("Error {} file '{}'.", "deleting".red().bold(), path.italic())
    }
}

fn is_file(path: String) -> bool {
    metadata(path).unwrap().is_file()
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        exit(0)
    }

    args.remove(0);
    for arg in args {
        let path: String = arg.to_owned();

        if is_file(arg) {
            delete_file(path);
        }
        else {
            delete_folder(path);
        }
    }
}
