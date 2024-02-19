extern crate colored;

use chrono::offset::Utc;
use colored::Colorize;
use std::env;

fn main() {
    let mut format = "%H:%M:%S %d/%m/%Y";
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        format = args[1].as_str();
    }

    println!("{}", Utc::now().format(format).to_string().bold());
}
