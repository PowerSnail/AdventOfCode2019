extern crate clap;
mod intcode_machine;
mod util;

use intcode_machine::{run_all, Machine, State};
use std::collections::HashMap;
use std::io::{stdin, BufRead};
use util::{PartID, part_id_from_cli};

struct Frame {
    map: HashMap<(i64, i64), i64>,
    max_x: i64,
    max_y: i64,
    score: i64,
    ball_x : i64,
    bar_x: i64,
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            map: HashMap::new(),
            max_x: -1,
            max_y: -1,
            score: -1,
            ball_x: -1,
            bar_x: -1,
        }
    }
}

fn apply_change(machine: &mut Machine, frame: &mut Frame) {
    while machine.has_output() {
        let x = machine.pop_output();
        let y = machine.pop_output();
        let tile = machine.pop_output();

        if x == -1 {
            frame.score = tile;
            continue;
        }

        if tile == 3 {
            frame.bar_x = x;
        }
        if tile == 4 {
            frame.ball_x = x;
        }

        frame.map.insert((x, y), tile);
        frame.max_x = frame.max_x.max(x);
        frame.max_y = frame.max_y.max(y);
    }
}

fn render(frame: &Frame) {
    for y in 0..=frame.max_y {
        for x in 0..=frame.max_x {
            let &tile = frame.map.get(&(x, y)).unwrap_or(&0);
            print!(
                "{}",
                match tile {
                    0 => "  ",
                    1 => "WW",
                    2 => "**",
                    3 => "__",
                    4 => "()",
                    _ => panic!("Wrong tile"),
                }
            );
        }
        println!("");
    }
    println!("Score = {}", frame.score);
}

fn load_machine() -> Machine {
    let program = stdin().lock()
        .split(',' as u8)
        .map(|chunk| String::from_utf8(chunk.ok()?).ok()?.parse().ok())
        .map(|result| result.expect("Failed to get next input"))
        .collect();

    Machine::new(&program)
}

fn autoplay(frame: &Frame) -> i64 {
    frame.ball_x - frame.bar_x
}

fn main() {
    let mut machine = load_machine();
    let mut frame = Frame::new();
    match part_id_from_cli() {
        PartID::One => {
            run_all(&mut machine, yield_iter![]);
            apply_change(&mut machine, &mut frame);
            let result = frame.map.iter().filter(|&(_, v)| *v == 2).count();
            println!("{}", result);
        }
        PartID::Two => {
            machine.memset(0, 2);
            let mut state = run_all(&mut machine, yield_iter![]);
            apply_change(&mut machine, &mut frame);
            while state != State::Halted {
                let joy_stick = autoplay(&frame);
                state = run_all(&mut machine, yield_iter![joy_stick, ]);
                apply_change(&mut machine, &mut frame);
            }
            println!("{}", frame.score);
        }
    }
}
