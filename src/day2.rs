mod util;

use std::io::BufRead;
use util::error_exit;


fn process(memory: &mut [usize], cursor: usize) {
    let op = memory[cursor];
    match op {
        1 => {
            let v1 = memory[memory[cursor + 1]];
            let v2 = memory[memory[cursor + 2]];
            memory[memory[cursor + 3]] = v1 + v2;
            process(memory, cursor + 4);
        },
        2 => {
            let v1 = memory[memory[cursor + 1]];
            let v2 = memory[memory[cursor + 2]];
            memory[memory[cursor + 3]] = v1 * v2;
            process(memory, cursor + 4);
        },
        99 => (),
        _ => error_exit("Error in reading program")
    }
}


fn main() {

    let part = util::part_id_from_cli();

    let mut memory: Vec<usize> = std::io::stdin()
        .lock()
        .lines()
        .map(or_abort!("Failed to read from stdin"))
        .nth(0)
        .unwrap()
        .split(",")
        .map(|line| line.parse::<usize>())
        .map(or_abort!("Failed to parse"))
        .collect()
        ;

    match part {
        util::PartID::One => {
            memory[1] = 12;
            memory[2] = 2;
            process(&mut memory, 0);
            println!("{}", memory[0]);
        },
        util::PartID::Two => {
            let output: usize = 19690720;
            let max_pos = memory.len();

            // Naive solution
            for i in 0..max_pos {
                for j in 0..max_pos {
                    let mut local_memory = memory.clone();
                    local_memory[1] = i;
                    local_memory[2] = j;
                    process(&mut local_memory, 0);
                    if local_memory[0] == output {
                        println!("{}", local_memory[1] * 100 + local_memory[2]);
                        return;
                    }
                }
            }
            println!("Search done. Not found.");            
        }
    }
}
