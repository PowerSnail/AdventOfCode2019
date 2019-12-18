mod intcode_machine;
mod util;

use intcode_machine::{run_all, Machine, State, ValueType};
use std::collections::HashMap;
use std::io::BufRead;
use util::{error_exit, part_id_from_cli, PartID};

const BLACK: ValueType = 0;
const WHITE: ValueType = 1;
const LEFT: ValueType = 0;
const RIGHT: ValueType = 1;

const DIR_UP: ValueType = 0;
const DIR_RIGHT: ValueType = 1;
const DIR_DOWN: ValueType = 2;
const DIR_LEFT: ValueType = 3;

struct Robot {
    x: i64,
    y: i64,
    direction: ValueType,
}

impl Robot {
    fn step(&self, turn: ValueType) -> Robot {
        let direction = match turn {
            LEFT => (self.direction + 4 - 1) % 4,
            RIGHT => (self.direction + 1) % 4,
            _ => error_exit("Turn is neither left nor right"),
        };
        let (x, y) = match direction {
            DIR_UP => (self.x, self.y - 1),
            DIR_RIGHT => (self.x + 1, self.y),
            DIR_DOWN => (self.x, self.y + 1),
            DIR_LEFT => (self.x - 1, self.y),
            _ => unreachable!(),
        };
        Robot { x, y, direction }
    }

    fn new() -> Robot {
        Robot {
            x: 0,
            y: 0,
            direction: DIR_UP,
        }
    }
}

fn main() {
    let program: Vec<ValueType> = lines_from_stdin!()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|code| match code.parse() {
            Ok(v) => v,
            Err(e) => {
                error_exit(&format!("Failed to parse {}. Error = {:#}", code, e));
            }
        })
        .collect();
    let mut machine = Machine::new(&program);
    let mut robot = Robot::new();
    let mut map: HashMap<(i64, i64), i64> = HashMap::new();

    let part = part_id_from_cli();
    match part {
        PartID::One => (),
        PartID::Two => {
            map.insert((0, 0), WHITE);
        }
    };

    loop {
        let cur_color = map.get(&(robot.x, robot.y)).unwrap_or(&BLACK);
        let state = run_all(&mut machine, yield_iter![*cur_color,]);
        let color = machine.pop_output();
        let direction = machine.pop_output();
        map.entry((robot.x, robot.y))
            .and_modify(|e| *e = color)
            .or_insert(color);
        robot = robot.step(direction);
        match state {
            State::Halted => break,
            _ => (),
        };
    }

    match part {
        PartID::One => println!("{}", map.len()),
        PartID::Two => {
            let min_x = map.keys().map(|&(x, _)| x).min().unwrap();
            let max_x = map.keys().map(|&(x, _)| x).max().unwrap();
            let min_y = map.keys().map(|&(_, y)| y).min().unwrap();
            let max_y = map.keys().map(|&(_, y)| y).max().unwrap();

            for y in min_y..=max_y {
                let mut line = String::new();
                for x in min_x..=max_x {
                    line.push_str(match map.get(&(x, y)) {
                        Some(&c) if c == WHITE => "**",
                        _ => "  ",
                    })
                }
                println!("{}", &line);
            }
        }
    };
}
