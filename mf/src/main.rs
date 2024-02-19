extern crate colored;

use colored::Colorize;

use std::env;
use std::fs::File;
use std::process::exit;

fn create_file(path: String) {
    match File::create(path.clone()) {
        Ok(_)=> {
            println!("File '{}' successfully {}.",
                path.italic(),
                "created".to_string().green().bold());
        },
        Err(_)=> {
            println!("{} to create file '{}'.",
                "Failed".to_string().red().bold(),
                path.italic());
        }
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        exit(0)
    }

    args.remove(0);
    for arg in args {
        create_file(arg);
    }
}
