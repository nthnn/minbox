extern crate colored;

use colored::Colorize;
use std::env;
use std::io::Error;
use std::path::PathBuf;

fn main() {
    let curdir: Result<PathBuf, Error> = env::current_dir();

    match curdir {
        Ok(wd)=> {
            println!("{}", wd.display().to_string().bold());
        },
        Err(_)=> {
            eprintln!("Something went {}.", "wrong".red().bold());
        }
    }
}