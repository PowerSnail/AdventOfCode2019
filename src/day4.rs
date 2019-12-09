mod util;

use std::io::BufRead;
use util::{error_exit, part_id_from_cli, PartID};

fn is_valid(num: i32, require_absolute_2: bool) -> bool {
    let digits: Vec<i32> = vec![
        num / 100000,
        num / 10000 % 10,
        num / 1000 % 10,
        num / 100 % 10,
        num / 10 % 10,
        num % 10,
    ];
    let sorted = digits[1..]
        .iter()
        .zip(digits.iter())
        .map(|(curr, prev)| prev <= curr)
        .fold(true, |a, b| a && b);

    if !sorted {
        return false;
    }
    let mut accumulator = vec![1];
    digits[1..]
        .iter()
        .zip(digits.iter())
        .for_each(|(curr, prev)| match curr == prev {
            true => {
                let last = accumulator.len() - 1;
                accumulator[last] += 1;
            }
            false => accumulator.push(1),
        });

    if require_absolute_2 {
        return accumulator.into_iter().any(|c| c == 2);
    } else {
        return accumulator.into_iter().any(|c| c >= 2);
    }
}

fn main() {
    let (lo, hi) = match lines_from_stdin!()
        .take(2)
        .map(|line| line.parse())
        .map(or_abort!("Failed to parse"))
        .collect::<Vec<i32>>()[..]
    {
        [lo, hi] => (lo, hi),
        _ => unreachable!(),
    };

    let absolute_2 = match part_id_from_cli() {
        PartID::One => false,
        PartID::Two => true,
    };

    let result = (lo..=hi).filter(|&x| is_valid(x, absolute_2)).count();

    println!("{}", result);
}
