use std::env;
use std::process::exit;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        exit(0)
    }

    args.remove(0);
    for arg in args {
        print!("{} ", arg)
    }

    println!()
}
