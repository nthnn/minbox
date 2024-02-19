extern crate chrono;

mod file;
mod util;

use std::env;

fn main() {
    let mut entry_path: &str = ".";
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        entry_path = args[1].as_str();
    }

    for local_file in file::list_files(entry_path) {
        file::print_file(local_file);
    }
}
