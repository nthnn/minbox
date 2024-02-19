extern crate clap;

mod args;
mod printer;
mod reader;

use clap::*;
use colored::Colorize;
use std::process::exit;

fn main() {
    let args: ArgMatches = args::parse_args();

    let with_lines: bool = match args.get_one::<bool>("with_lines") {
        Some(&true)=> true,
        Some(&false)=> false,
        None=> false
    };

    let from: i32 = match args.get_one("from") {
        Some(value)=> *value,
        None=> 0
    };

    let to: i32 = match args.get_one("to") {
        Some(value)=> *value,
        None=> 0
    };

    let file: &String = match args.get_one("file") {
        Some(value)=> value,
        None=> {
            eprintln!("No input {}.", "files".red().bold());
            exit(0)
        }
    };

    let filename: String = file.to_string();
    printer::print_read_data(filename, reader::ReadOptions {
        from: from as u32,
        to: to as u32,
        with_line_num: with_lines
    });
}
