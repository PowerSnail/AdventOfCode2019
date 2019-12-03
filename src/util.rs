extern crate clap;

use clap::{App, Arg};
use std::cmp;

pub fn error_exit(msg: &str) -> ! {
    eprintln!("{}", msg);
    std::process::exit(-1);
}

pub fn clip_min<T: Ord>(value: T, min_v: T) -> T {
    cmp::max(min_v, value)
}

#[macro_export]
macro_rules! or_abort {
    ($msg:expr) => {
        |result| match result {
            Ok(value) => value,
            Err(_) => error_exit($msg),
        }
    };
}

#[macro_export]
macro_rules! lines_from_stdin {
    () => {
        std::io::stdin()
        .lock()
        .lines()
        .map(or_abort!("Failed to read"))
    };
}

pub enum PartID {
    One,
    Two,
}

pub fn part_id_from_cli() -> PartID {
    let args = App::new("Day1")
        .arg(
            Arg::with_name("part")
                .possible_value("part1")
                .possible_value("part2"),
        )
        .get_matches();
    match args.value_of("part") {
        Some("part1") => PartID::One,
        Some("part2") => PartID::Two,
        _ => panic!("Error in part"),
    }
}

