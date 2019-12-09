mod util;

use std::io::BufRead;
use util::error_exit;

const TENS: [i32; 3] = [100, 1000, 10000];

const ADD: i32 = 1;
const MULTIPLY: i32 = 2;
const INPUT: i32 = 3;
const PRINT: i32 = 4;
const JMP_IF: i32 = 5;
const JMP_IF_NOT: i32 = 6;
const CMP_LT: i32 = 7;
const CMP_EQ: i32 = 8;
const HALT: i32 = 99;

macro_rules! def_param {
    (Value $var:ident = $mem:expr, $pc:expr, $i:expr) => {
        let $var: i32 = match $mem[$pc] / TENS[$i - 1] % 10 {
            0 => $mem[$mem[$pc + $i] as usize],
            1 => $mem[$pc + $i],
            _ => error_exit("Invalid mode"),
        };
    };
    (OutAddr $var:ident = $mem: expr, $pc: expr, $i:expr) => {
        let $var: usize =$mem[$pc + $i] as usize;
    };
}

fn process<T>(memory: &mut [i32], cursor: usize, mut input: T)
where
    T: Iterator<Item = i32>,
{
    eprint!("[{:03}] Op = {:05} | ", cursor,  memory[cursor]);

    let cursor: usize = match  memory[cursor as usize] % 100 {
        ADD => {
            def_param!(Value v1 = memory, cursor, 1);
            def_param!(Value v2 = memory, cursor, 2);
            def_param!(OutAddr p_out = memory, cursor, 3);
            memory[p_out] = v1 + v2;
            eprintln!("ADD {} {} mem[{}] = {}", v1, v2, p_out, memory[p_out]);
            cursor + 4
        }
        MULTIPLY => {
            def_param!(Value v1 = memory, cursor, 1);
            def_param!(Value v2 = memory, cursor, 2);
            def_param!(OutAddr p_out = memory, cursor, 3);
            memory[p_out] = v1 * v2;
            eprintln!("MULTIPLY {} {} mem[{}] = {}", v1, v2, p_out, memory[p_out]);
            cursor + 4
        }
        INPUT => {
            def_param!(OutAddr p_out = memory, cursor, 1);
            memory[p_out] = input.next().unwrap();
            eprintln!("INPUT mem[{}] = {}", p_out, memory[p_out]);
            cursor + 2
        }
        PRINT => {
            def_param!(Value v = memory, cursor, 1);
            eprintln!("PRINT {}", v);
            println!("-> {}", v);
            cursor + 2
        }
        JMP_IF => {
            def_param!(Value v = memory, cursor, 1);
            def_param!(Value addr = memory, cursor, 2);
            eprintln!("JMP_IF cond={}, to={}", v, addr as usize);
            match v {
                0 => cursor + 3,
                _ => addr as usize
            }
        },
        JMP_IF_NOT => {
            def_param!(Value v = memory, cursor, 1);
            def_param!(Value addr = memory, cursor, 2);
            eprintln!("JMP_IF_NOT cond={}, to={}", v, addr as usize);
            match v {
                0 => addr as usize,
                _ => cursor + 3
            }
        },
        CMP_LT => {
            def_param!(Value v1 = memory, cursor, 1);
            def_param!(Value v2 = memory, cursor, 2);
            def_param!(OutAddr p_out = memory, cursor, 3);
            memory[p_out] = match v1 < v2 {
                true => 1,
                false => 0
            };
            eprintln!("CMP_LT {} {} mem[{}]={}", v1, v2, p_out, memory[p_out]);
            cursor + 4
        },
        CMP_EQ => {
            def_param!(Value v1 = memory, cursor, 1);
            def_param!(Value v2 = memory, cursor, 2);
            def_param!(OutAddr p_out = memory, cursor, 3);
            memory[p_out] = match v1 == v2 {
                true => 1,
                false => 0
            };
            eprintln!("CMP_EQ {} {} mem[{}]={}", v1, v2, p_out, memory[p_out]);
            cursor + 4
        },
        HALT => return,
        _ => error_exit(&format!("Invalid command {} at {}", memory[cursor], cursor)),
    };
    process(memory, cursor, input);
}

fn main() {
    let mut program: Vec<i32> = lines_from_stdin!().nth(0).unwrap()
        .split(',')
        .map(|code| match code.parse() {
            Ok(v) => v,
            Err(e) => {
                error_exit(&format!("Failed to parse {}. Error = {:#}", code, e));
            }
        })
        .collect();

    let user_inputs = match util::part_id_from_cli() {
        util::PartID::One => [1],
        util::PartID::Two => [5]
    };

    process(&mut program, 0, user_inputs.into_iter().map(|&x| x));
}
