mod util;

use std::collections::VecDeque;
use std::io::BufRead;
use util::{error_exit, part_id_from_cli, PartID};

type ValueType = i64;


const TENS: [ValueType; 3] = [100, 1000, 10000];

const ADD: ValueType = 1;
const MULTIPLY: ValueType = 2;
const INPUT: ValueType = 3;
const OUTPUT: ValueType = 4;
const JMP_IF_NON_ZERO: ValueType = 5;
const JMP_IF_ZERO: ValueType = 6;
const CMP_LT: ValueType = 7;
const CMP_EQ: ValueType = 8;
const MOVE_RBASE: ValueType = 9;
const HALT: ValueType = 99;

const MODE_POSITION: ValueType = 0;
const MODE_IMMEDIATE: ValueType = 1;
const MODE_RELATIVE: ValueType = 2;

#[derive(Debug)]
struct Machine {
    memory: Vec<ValueType>,
    cursor: usize,
    relative_base: ValueType,
    in_queue: VecDeque<ValueType>,
    out_queue: VecDeque<ValueType>,
    debug_mode: bool,
}

#[derive(Debug)]
enum State {
    Halted,
    Running,
    InputBlock,
}

impl Machine {
    fn new(init_mem: &Vec<ValueType>) -> Machine {
        Machine {
            memory: init_mem.clone(),
            cursor: 0,
            relative_base: 0,
            in_queue: VecDeque::new(),
            out_queue: VecDeque::new(),
            debug_mode: false,
        }
    }

    fn param_val(&mut self, index: usize) -> ValueType {
        let immediate_val = self.memory[self.cursor + index + 1];
        self.debug(&format!(
            "PARAM : immediate val = {}, mode = {}",
            immediate_val,
            self.memory[self.cursor] / TENS[index] % 10
        ));
        match self.memory[self.cursor] / TENS[index] % 10 {
            MODE_POSITION => self.load(immediate_val),
            MODE_RELATIVE => {
                let addr = self.as_addr(self.relative_base + immediate_val);
                self.memory[addr]
            },
            MODE_IMMEDIATE => immediate_val,
            _ => error_exit("Invalid mode code"),
        }
    }

    fn param_out_addr(&mut self, index: usize) -> usize {
        let immediate_val = self.memory[self.cursor + index + 1];
        self.debug(&format!(
            "OUT ADDR : immediate val = {}, mode = {}",
            immediate_val,
            self.memory[self.cursor] / TENS[index] % 10
        ));
        self.as_addr(match self.memory[self.cursor] / TENS[index] % 10 {
            MODE_POSITION => immediate_val,
            MODE_RELATIVE => self.relative_base + immediate_val,
            _ => error_exit("Invalid mode code"),
        })
    }

    fn as_addr(&mut self, val: ValueType) -> usize {
        let val = val as usize;
        if val >= self.memory.len() {
            self.memory.resize(val + 1, 0);
        }

        val
    }

    fn load(&mut self, addr: ValueType) -> ValueType {
        let addr = self.as_addr(addr);
        self.memory[addr]
    }

    fn debug(&mut self, msg: &str) {
        match self.debug_mode {
            true => eprintln!(
                "DEBUG [{} {} {} {}] {}",
                self.cursor,
                self.relative_base,
                self.in_queue.len(),
                self.out_queue.len(),
                msg
            ),
            false => (),
        }
    }
}

fn add(m: &mut Machine) -> State {
    let v1 = m.param_val(0);
    let v2 = m.param_val(1);
    let p_out = m.param_out_addr(2);
    m.memory[p_out] = v1 + v2;
    m.debug(&format!(
        "ADD {} + {} => {} = {}",
        v1, v2, p_out, m.memory[p_out]
    ));
    m.cursor += 4;
    State::Running
}

fn multiply(m: &mut Machine) -> State {
    let v1 = m.param_val(0);
    let v2 = m.param_val(1);
    let p_out = m.param_out_addr(2);
    m.debug(&format!(
        "MULTI {} * {} => {}",
        v1, v2, p_out
    ));
    m.memory[p_out] = v1 * v2;
    m.cursor += 4;
    State::Running
}

fn save(m: &mut Machine) -> State {
    match m.in_queue.pop_front() {
        None => State::InputBlock,
        Some(input_val) => {
            let p_out = m.param_out_addr(0);
            m.memory[p_out] = input_val;
            m.cursor += 2;
            m.debug(&format!("INPUT Save {} -> {}", input_val, p_out));
            State::Running
        }
    }
}

fn print(m: &mut Machine) -> State {
    let v = m.param_val(0);
    m.out_queue.push_back(v);
    m.debug(&format!("PRINT {}", v));
    m.cursor += 2;
    State::Running
}

fn jmp_if_non_zero(m: &mut Machine) -> State {
    let v = m.param_val(0);
    let destination = m.param_val(1) as usize;
    m.debug(&format!("JMP IF NON ZERO {} to {}", v, destination));
    m.cursor = match v {
        0 => m.cursor + 3,
        _ => destination,
    };
    State::Running
}

fn jmp_if_zero(m: &mut Machine) -> State {
    let v = m.param_val(0);
    let destination = m.param_val(1) as usize;
    m.debug(&format!("JMP IF ZERO {} to {}", v, destination));
    m.cursor = match v {
        0 => destination,
        _ => m.cursor + 3,
    };
    State::Running
}

fn cmp_lt(m: &mut Machine) -> State {
    let v1 = m.param_val(0);
    let v2 = m.param_val(1);
    let p_out = m.param_out_addr(2);
    m.debug(&format!("CMP LT {} <=> {} -> {}", v1, v2, p_out));
    m.memory[p_out] = match v1 < v2 {
        true => 1,
        false => 0,
    };
    m.cursor += 4;
    State::Running
}

fn cmp_eq(m: &mut Machine) -> State {
    let v1 = m.param_val(0);
    let v2 = m.param_val(1);
    let p_out = m.param_out_addr(2);
    m.debug(&format!("CMP EQ {} <=> {} -> {}", v1, v2, p_out));
    m.memory[p_out] = match v1 == v2 {
        true => 1,
        false => 0,
    };
    m.cursor += 4;
    State::Running
}

fn move_rbase(m: &mut Machine) -> State {
    let v1 = m.param_val(0);
    m.debug(&format!("MOVE RBASE {} ", v1));
    m.relative_base += v1;
    m.cursor += 2;
    State::Running
}

fn step(m: &mut Machine) -> State {
    match m.memory[m.cursor] % 100 {
        ADD => add(m),
        MULTIPLY => multiply(m),
        INPUT => save(m),
        OUTPUT => print(m),
        JMP_IF_NON_ZERO => jmp_if_non_zero(m),
        JMP_IF_ZERO => jmp_if_zero(m),
        CMP_LT => cmp_lt(m),
        CMP_EQ => cmp_eq(m),
        MOVE_RBASE => move_rbase(m),
        HALT => State::Halted,
        invalid_code => error_exit(&format!("Invalid command {} at {}", invalid_code, m.cursor)),
    }
}

fn run_all<T>(m: &mut Machine, input: T) -> State 
    where T: Iterator<Item=ValueType>
{
    for v in input {
        m.in_queue.push_back(v);
    }
    loop {
        match step(m) {
            State::Running => (),
            State::Halted => return State::Halted,
            State::InputBlock => return State::InputBlock,
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

    let part = part_id_from_cli();
    let mut m: Machine = Machine::new(&program);
    let input_code = match part {
        PartID::One => 1,
        PartID::Two => 2,
    };
    match run_all(&mut m, yield_iter![input_code,]) {
        State::Halted => println!("{}", m.out_queue.pop_front().unwrap()),
        s => println!("{:?}", s),
    };
}
