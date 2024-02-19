extern crate colored;

use crate::reader;
use colored::Colorize;
use std::process::exit;

fn pad_str(mut s: String, len: usize) -> String {
    while s.len() < len {
        s.push(' ');
    }

    s
}

pub fn print_read_data(filename: String, options: reader::ReadOptions) {
    if options.to <= options.from && (options.to != 0 && options.from != 0) {
        eprintln!("{}", "End line number must be greater than starting line number.".red().bold());
        exit(0);
    }

    let contents: String = reader::read_file(filename.to_owned());
    let lines: Vec<&str> = contents.split('\n').collect();
    let mut idx: u32 = 1;
    let mut line_idx: u32 = 1;

    for line in lines {
        line_idx += 1;
        idx += 1;

        if options.from != 0 && line_idx - 1 < options.from {
            continue;
        }
        else if options.to != 0 && line_idx - 1 > options.to {
            break;
        }

        if options.with_line_num {
            print!("{} {} ", pad_str((idx - 1).to_string(), 5).cyan(), "|".cyan().bold());
        }

        println!("{}", line);
    }
}