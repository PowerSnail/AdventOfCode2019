extern crate clap;

use clap::{App, Arg};
use std::cmp;
// use std::str::FromStr;

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

// pub fn clip_max<T: Ord>(value: T, max_v: T) -> T {
//     cmp::min(max_v, value)
// }
