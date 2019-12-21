extern crate regex;
mod util;

use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::BufRead;
use util::{error_exit, part_id_from_cli, PartID};

macro_rules! v_add {
    ($v1:expr, $v2:expr) => {
        $v1.iter()
            .zip($v2.iter())
            .map(|(&x1, &x2)| x1 + x2)
            .collect()
    };
}

macro_rules! abs_sum {
    ($head:expr $(, $v:expr)*) => {
        $head.abs() $(+ $v.abs())*
    };
}

fn gcd(x: i64, y: i64) -> i64 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn lcm(x: i64, y: i64) -> i64 {
    x * y / gcd(x, y)
}

fn delta_v(p: &Vec<i64>) -> Vec<i64> {
    let count = p.len();
    (0..count)
        .map(|i| {
            (0..count)
                .filter(|&j| j != i)
                .map(|j| match p[j].cmp(&p[i]) {
                    Ordering::Greater => 1,
                    Ordering::Equal => 0,
                    Ordering::Less => -1,
                })
                .sum()
        })
        .collect()
}

fn parse_input() -> Vec<Vec<i64>> {
    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").expect("Failed to build regex");

    lines_from_stdin!()
        .map(|line| {
            let groups = match re.captures(&line) {
                Some(groups) => groups,
                None => error_exit(&format!("Failed to parse line {}", &line)),
            };
            let parsed: Vec<i64> = (0..3)
                .map(|dim| {
                    groups
                        .get(dim + 1)
                        .unwrap()
                        .as_str()
                        .parse()
                        .expect("Failed to parse")
                })
                .collect();
            parsed
        })
        .collect()
}

fn transpose(v: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut y: Vec<Vec<i64>> = Vec::new();
    y.resize_with(v[0].len(), || [0].repeat(v.len()));

    for i in 0..v.len() {
        for dim in 0..v[0].len() {
            y[dim][i] = v[i][dim];
        }
    }
    y
}

fn step_until_repeat(position: &Vec<i64>) -> i64 {
    let mut position = position.clone();
    let mut velocity: Vec<i64> = [0].repeat(position.len());
    let mut existing_config: HashSet<(Vec<i64>, Vec<i64>)> = HashSet::new();

    let mut iteration = 0;
    loop {
        let config = (velocity.clone(), position.clone());
        if existing_config.contains(&config) {
            return iteration;
        }
        existing_config.insert(config);

        let dv = delta_v(&position);
        velocity = v_add!(velocity, dv);
        position = v_add!(position, velocity);
        iteration += 1;
    }
}

fn main() {
    let mut positions = transpose(parse_input());
    let count = positions[0].len();

    match part_id_from_cli() {
        PartID::One => {
            let mut velocity: Vec<Vec<i64>> = Vec::new();
            velocity.resize_with(3, || [0].repeat(count));

            for dim in 0..3 {
                for _ in 0..1000 {
                    let dv = delta_v(&positions[dim]);
                    velocity[dim] = v_add!(velocity[dim], dv);
                    positions[dim] = v_add!(positions[dim], velocity[dim]);
                }
            }
            let mut total: i64 = 0;
            for i in 0..count {
                let pot: i64 = abs_sum!(positions[0][i], positions[1][i], positions[2][i]);
                let kin: i64 = abs_sum!(velocity[0][i], velocity[1][i], velocity[2][i]);
                total += pot * kin;
            }
            println!("{}", total);
        }
        PartID::Two => {
            let n_step = (0..3)
                .map(|i| step_until_repeat(&positions[i]))
                .fold(1, |prev, v| lcm(prev, v));
            println!("{}", n_step);
        }
    }
}
