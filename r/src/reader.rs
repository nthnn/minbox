use colored::Colorize;
use std::fs;
use std::process::exit;

pub struct ReadOptions {
    pub from:           u32,
    pub to:             u32,
    pub with_line_num:  bool
}

pub fn read_file(filename: String) -> String {
    match fs::read_to_string(filename) {
        Ok(contents)=> contents,
        Err(_)=> {
            eprintln!("{}", "Error reading file.".red().bold());
            exit(0);
        }
    }
}