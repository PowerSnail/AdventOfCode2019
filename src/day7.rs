mod util;

use std::collections::VecDeque;
use std::io::BufRead;
use util::{error_exit, part_id_from_cli, permute, PartID};

const TENS: [i32; 3] = [100, 1000, 10000];

const ADD: i32 = 1;
const MULTIPLY: i32 = 2;
const INPUT: i32 = 3;
const OUTPUT: i32 = 4;
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
        let $var: usize = $mem[$pc + $i] as usize;
    };
}

struct Machine {
    memory: Vec<i32>,
    cursor: usize,
    output: VecDeque<i32>,
}

enum State {
    Halted,
    Running,
}

impl Machine {
    fn new(init_mem: &Vec<i32>) -> Machine {
        Machine {
            memory: init_mem.clone(),
            cursor: 0,
            output: VecDeque::new(),
        }
    }

    fn continue_until_blocked<T>(&mut self, mut input: T) -> State
    where
        T: Iterator<Item = i32>,
    {
        loop {
            let cursor: usize = match self.memory[self.cursor as usize] % 100 {
                ADD => {
                    def_param!(Value v1 = self.memory, self.cursor, 1);
                    def_param!(Value v2 = self.memory, self.cursor, 2);
                    def_param!(OutAddr p_out = self.memory, self.cursor, 3);
                    self.memory[p_out] = v1 + v2;
                    // eprintln!("ADD {} {} mem[{}] = {}", v1, v2, p_out, self.memory[p_out]);
                    self.cursor + 4
                }
                MULTIPLY => {
                    def_param!(Value v1 = self.memory, self.cursor, 1);
                    def_param!(Value v2 = self.memory, self.cursor, 2);
                    def_param!(OutAddr p_out = self.memory, self.cursor, 3);
                    self.memory[p_out] = v1 * v2;
                    // eprintln!("MULTIPLY {} {} mem[{}] = {}", v1, v2, p_out, self.memory[p_out]);
                    self.cursor + 4
                }
                INPUT => {
                    let input_val = match input.next() {
                        Some(v) => v,
                        None => return State::Running,
                    };
                    def_param!(OutAddr p_out = self.memory, self.cursor, 1);
                    self.memory[p_out] = input_val;
                    // eprintln!("INPUT mem[{}] = {}", p_out, self.memory[p_out]);
                    self.cursor + 2
                }
                OUTPUT => {
                    def_param!(Value v = self.memory, self.cursor, 1);
                    // eprintln!("OUTPUT {}", v);
                    self.output.push_back(v);
                    self.cursor + 2
                }
                JMP_IF => {
                    def_param!(Value v = self.memory, self.cursor, 1);
                    def_param!(Value addr = self.memory, self.cursor, 2);
                    // eprintln!("JMP_IF cond={}, to={}", v, addr as usize);
                    match v {
                        0 => self.cursor + 3,
                        _ => addr as usize,
                    }
                }
                JMP_IF_NOT => {
                    def_param!(Value v = self.memory, self.cursor, 1);
                    def_param!(Value addr = self.memory, self.cursor, 2);
                    // eprintln!("JMP_IF_NOT cond={}, to={}", v, addr as usize);
                    match v {
                        0 => addr as usize,
                        _ => self.cursor + 3,
                    }
                }
                CMP_LT => {
                    def_param!(Value v1 = self.memory, self.cursor, 1);
                    def_param!(Value v2 = self.memory, self.cursor, 2);
                    def_param!(OutAddr p_out = self.memory, self.cursor, 3);
                    self.memory[p_out] = match v1 < v2 {
                        true => 1,
                        false => 0,
                    };
                    // eprintln!("CMP_LT {} {} mem[{}]={}", v1, v2, p_out, self.memory[p_out]);
                    self.cursor + 4
                }
                CMP_EQ => {
                    def_param!(Value v1 = self.memory, self.cursor, 1);
                    def_param!(Value v2 = self.memory, self.cursor, 2);
                    def_param!(OutAddr p_out = self.memory, self.cursor, 3);
                    self.memory[p_out] = match v1 == v2 {
                        true => 1,
                        false => 0,
                    };
                    // eprintln!("CMP_EQ {} {} mem[{}]={}", v1, v2, p_out, self.memory[p_out]);
                    self.cursor + 4
                }
                HALT => {
                    // eprintln!("HALT!");
                    return State::Halted;
                }
                _ => error_exit(&format!(
                    "Invalid command {} at {}",
                    self.memory[self.cursor], self.cursor
                )),
            };
            self.cursor = cursor;
        }
    }
}

fn get_output(phases: &Vec<i32>, init_mem: &Vec<i32>) -> i32 {
    let m_count = phases.len();
    let mut m_list: Vec<Machine> = Vec::new();
    for &p in phases {
        let mut machine = Machine::new(init_mem);
        machine.continue_until_blocked(yield_iter![p,]);
        m_list.push(machine);
    }

    let mut value: i32 = 0;
    let mut halted_count: usize = 0;
    let mut current_machine: usize = 0;
    while halted_count < m_count {
        match m_list[current_machine].continue_until_blocked(yield_iter![value,]) {
            State::Halted => halted_count += 1,
            _ => (),
        };

        value = m_list[current_machine].output.pop_front().unwrap();
        current_machine = (current_machine + 1) % m_count;
    }
    value
}

fn main() {
    let program: Vec<i32> = lines_from_stdin!()
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
    let part = part_id_from_cli();
    let result = match part {
        PartID::One => permute(0..=4)
            .map(|phase| get_output(&phase, &program))
            .max()
            .unwrap(),
        PartID::Two => permute(5..=9)
            .map(|phase| get_output(&phase, &program))
            .max()
            .unwrap(),
    };
    println!("Result = {}", result);
}
