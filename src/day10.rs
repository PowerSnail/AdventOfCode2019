mod util;

use std::collections::HashSet;
use std::io::BufRead;

use util::{error_exit, part_id_from_cli, PartID, collect_sorted_vec};

fn gcd(x: i64, y: i64) -> i64 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn simplest_vector(numerator: i64, denominator: i64) -> (i64, i64) {
    match (numerator.abs(), denominator.abs()) {
        (0, 0) => (0, 0),
        (0, d) => (0, d / denominator),
        (n, 0) => (n / numerator, 0),
        (n, d) => {
            let factor = gcd(d, n);
            (n * n / numerator / factor, d * d / denominator / factor)
        }
    }
}

fn lines_of_sight(asteroids: &Vec<(i64, i64)>) -> Vec<usize> {
    asteroids
        .iter()
        .map(|&p1| {
            let line_of_sight = asteroids
                .iter()
                .filter(|&p2| *p2 != p1)
                .map(|&p2| simplest_vector(p2.0 - p1.0, p2.1 - p1.1))
                .collect::<HashSet<(i64, i64)>>()
                .len();
            line_of_sight
        })
        .collect::<Vec<usize>>()
}

fn euclidean_to_polar((x, y): (i64, i64)) -> (i64, i64) {
    let x = x as f64;
    let y = y as f64;
    let angle = -x.atan2(y);
    let mag2 = x * x + y * y;
    ((angle * 10000.0) as i64, (mag2 * 10000.0) as i64)
}

fn nth_destroyed(n: usize, center: (i64, i64), asteroids: &Vec<(i64, i64)>) -> (i64, i64) {
    let (cx, cy) = center;
    let angle_magnitude = asteroids
        .iter()
        .map(|&(x, y)| ((x - cx, y - cy), (x, y)))
        .map(|(relative_coord, coord)| (euclidean_to_polar(relative_coord), coord));
    let angle_magnitude = collect_sorted_vec(angle_magnitude);

    let layer_angle = angle_magnitude
        .into_iter()
        .scan((-1, 0), |state, ((angle, _), coord)| {
            let layer = match state.0 == angle {
                true => state.1 + 1,
                false => 0,
            };

            *state = (angle, layer);
            Some(((layer, angle), coord))
        });
    let layer_angle = collect_sorted_vec(layer_angle);

    layer_angle[n].1
}

fn main() {
    let mut asteroids: Vec<(i64, i64)> = Vec::new();
    lines_from_stdin!().enumerate().for_each(|(y, line)| {
        line.char_indices()
            .filter(|&(_, c)| c == '#')
            .for_each(|(x, _)| asteroids.push((x as i64, y as i64)))
    });
    let lines = lines_of_sight(&asteroids);

    let result = match part_id_from_cli() {
        PartID::One => lines.into_iter().max().unwrap() as i64,
        PartID::Two => {
            let center_id = lines.iter().enumerate().max_by_key(|&(_, l)| l).unwrap().0;
            let center = asteroids.remove(center_id);
            let coord = nth_destroyed(200 - 1, center, &asteroids);
            coord.0 * 100 + coord.1
        }
    };

    println!("{}", result);
}
