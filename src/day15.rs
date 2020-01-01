mod intcode_machine;
mod util;

use intcode_machine::{run_all, Machine, State};
use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead};
use util::error_exit;

const C_NORTH: i64 = 1;
const C_SOUTH: i64 = 2;
const C_WEST: i64 = 3;
const C_EAST: i64 = 4;

const S_STILL: i64 = 0;
const S_MOVED: i64 = 1;
const S_FOUND: i64 = 2;

fn move_dir(cur_pos: &(i64, i64), dir: i64) -> (i64, i64) {
    match dir {
        C_NORTH => (cur_pos.0, cur_pos.1 - 1),
        C_SOUTH => (cur_pos.0, cur_pos.1 + 1),
        C_WEST => (cur_pos.0 - 1, cur_pos.1),
        C_EAST => (cur_pos.0 + 1, cur_pos.1),
        _ => error_exit("Wrong Direction"),
    }
}

struct WorldMap {
    machine: Machine,
    prev: HashMap<(i64, i64), i64>,
    wall: HashSet<(i64, i64)>,
    visited: HashSet<(i64, i64, i64)>,
    target: Option<(i64, i64)>,
}

impl WorldMap {
    fn new() -> WorldMap {
        let program = stdin()
            .lock()
            .split(',' as u8)
            .map(|chunk| String::from_utf8(chunk.ok()?).ok()?.parse().ok())
            .map(|result| result.expect("Failed to get next input"))
            .collect();

        WorldMap {
            machine: Machine::new(&program),
            prev: HashMap::new(),
            wall: HashSet::new(),
            target: None,
        }
    }

    fn step(&mut self, dir: i64) -> i64 {
        run_all(&mut self.machine, yield_iter![dir,]);
        self.machine.pop_output()
    }
}

fn visit(curr: (i64, i64), map: &mut WorldMap) {
    for &dir in &[C_NORTH, C_SOUTH, C_EAST, C_WEST] {
        let next = move_dir(&curr, dir);
        if map.wall.contains(&next) {
            continue;
        }
    }
}

fn main() {
    let mut my_world = WorldMap::new();
    visit((0, 0), &mut my_world);
}
