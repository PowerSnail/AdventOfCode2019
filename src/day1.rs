mod util;

use std::io::BufRead;
use util::error_exit;

fn get_fuel(mass: i64) -> i64 {
    util::clip_min(mass / 3 - 2, 0)
}

fn get_fuel_recursive(mass: i64) -> i64 {
    let fuel = get_fuel(mass);
    match fuel {
        0 => fuel,
        _ => fuel + get_fuel_recursive(fuel),
    }
}

fn main() {
    let solver = match util::part_id_from_cli() {
        util::PartID::One => get_fuel,
        util::PartID::Two => get_fuel_recursive,
    };

    let result: i64 = std::io::stdin()
        .lock()
        .lines()
        .map(or_abort!("Failed to read line"))
        .map(|line| line.parse::<i64>())
        .map(or_abort!("Failed to parse integer."))
        .map(solver)
        .sum();
    println!("{}", result)
}
