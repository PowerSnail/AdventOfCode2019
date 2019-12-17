mod util;

use std::io::{BufRead, Read};
use util::{error_exit, part_id_from_cli, PartID};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYER_SIZE: usize = WIDTH * HEIGHT;

fn part1(data: &str) -> usize {
    let n_layer = data.len() / LAYER_SIZE;
    (0..n_layer)
        .map(|l| {
            let mut counts = [0, 0, 0];
            let layer_range = l * LAYER_SIZE..(l + 1) * LAYER_SIZE;
            for c in data[layer_range].chars() {
                match c {
                    '0' => counts[0] += 1,
                    '1' => counts[1] += 1,
                    '2' => counts[2] += 1,
                    _ => (),
                };
            }
            (counts[0], counts[1] * counts[2])
        })
        .min()
        .unwrap()
        .1
}

const BLACK: char = '\u{25A1}';
const WHITE: char = '\u{25A0}';
const TRANS: char = 'T';

fn render() {
    let mut buffer: Vec<char> = Vec::new();
    buffer.resize(LAYER_SIZE, TRANS);
    std::io::stdin()
        .bytes()
        .map(|b| b.expect("Failed to read byte"))
        .enumerate()
        .for_each(|(i, b)| {
            match buffer[i % LAYER_SIZE] {
                TRANS => {
                    buffer[i % LAYER_SIZE] = match b as char {
                        '0' => BLACK,
                        '1' => WHITE,
                        '2' => TRANS,
                        _ => error_exit("Invalid char"),
                    }
                }
                _ => (),
            };
        });
    for line in 0..HEIGHT {
        println!(
            "{}",
            &buffer[line * WIDTH..(line + 1) * WIDTH]
                .iter()
                .collect::<String>()
        );
    }
}

fn main() {
    let part = part_id_from_cli();
    match part {
        PartID::One => println!("{}", part1(&lines_from_stdin!().nth(0).unwrap())),
        PartID::Two => render(),
    };
}
