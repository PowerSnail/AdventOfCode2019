#![allow(dead_code)]
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
            Err(e) => error_exit(&format!("{}\n{:#?}", $msg, e)),
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

#[macro_export]
macro_rules! yield_iter {
    [$($x:expr,)*] => {
        vec![$($x,)*].into_iter()
    };
}

pub struct Permutation<T> {
    array: Vec<T>,
    indexes: Vec<usize>,
    init: bool,
}

impl<T: Clone> Iterator for Permutation<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.init {
            self.init = false;
            return Some(self.array.clone());
        }
        let k = (0..self.array.len() - 1)
            .rev()
            .filter(|&k| self.indexes[k] < self.indexes[k + 1])
            .nth(0)?;
        let i = ((k + 1)..self.array.len())
            .rev()
            .filter(|&i| self.indexes[i] > self.indexes[k])
            .nth(0)?;

        self.indexes.swap(i, k);
        self.indexes[(k + 1)..].reverse();

        self.array.swap(i, k);
        self.array[(k + 1)..].reverse();

        Some(self.array.clone())
    }
}

pub fn permute<T, I>(number: I) -> Permutation<T>
where
    I: IntoIterator<Item = T>,
{
    let array: Vec<T> = number.into_iter().collect();
    let indexes: Vec<usize> = (0..array.len()).collect();
    Permutation {
        array,
        indexes,
        init: true,
    }
}
