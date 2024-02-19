extern crate clap;

use clap::*;

pub fn parse_args() -> ArgMatches {
    Command::new("rd")
        .disable_help_flag(true)
        .ignore_errors(true)        
        .arg(Arg::new("with_lines")
            .short('l')
            .long("with_lines")
            .action(ArgAction::SetTrue))
        .arg(Arg::new("from")
            .short('f')
            .long("from")
            .value_parser(value_parser!(i32))
            .action(ArgAction::Set))
        .arg(Arg::new("to")
            .short('t')
            .long("to")
            .value_parser(value_parser!(i32))
            .action(ArgAction::Set))
        .arg(Arg::new("file")
            .index(1))
        .get_matches()
}
