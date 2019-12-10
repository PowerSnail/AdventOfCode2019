mod util;

use std::io::BufRead;
use util::error_exit;

struct Point {
    x: i32,
    y: i32,
}

macro_rules! range_map {
    ($v1:expr => $v2:expr, $f:expr) => {
        if $v1 > $v2 {
            ($v2..$v1).rev().for_each($f)
        } else {
            (($v1 + 1)..=$v2).for_each($f)
        }
    };
}

macro_rules! line_map {
    ($p1:expr => $p2:expr, $f:expr) => {
        if $p1.x == $p2.x {
            range_map!($p1.y => $p2.y, |y| $f(Point {x: $p1.x, y: y}))
        } else if $p1.y == $p2.y {
            range_map!($p1.x => $p2.x, |x| $f(Point {x: x, y: $p1.y}))
        } else {
            panic!("No oblique lines");
        }
    };
}

macro_rules! map {
    (New $rect:expr, Fill $fill:expr) => {{
        let left = $rect.0;
        let bottom = $rect.1;
        let width = $rect.2 as usize;
        let height = $rect.3 as usize;
        let mut buffer = Vec::new();
        buffer.resize(width * height, $fill);
        (buffer, (left, bottom, width, height))
    }};
    ($map:expr, $x:expr, $y:expr) => {{
        let (left, bottom, width, _) = $map.1;
        let x = ($x - left) as usize;
        let y = ($y - bottom) as usize;
        $map.0[x + y * width]
    }};
    ($map:expr, $x:expr, $y:expr, Set $v:expr) => {{
        let (left, bottom, width, _) = $map.1;
        let x = ($x - left) as usize;
        let y = ($y - bottom) as usize;
        $map.0[x + y * width] = $v;
    }};
    ($map:expr, $p:expr) => {{
        map!($map, $p.x, $p.y)
    }};
    ($map:expr, $p:expr, Set $v:expr) => {{
        map!($map, $p.x, $p.y, Set $v);
    }};
}

fn parse_point(string: &str) -> Result<Point, ()> {
    match string.get(1..).unwrap().parse() {
        Ok(val) => match string.chars().next() {
            Some('U') => Ok(Point { x: 0, y: val }),
            Some('R') => Ok(Point { x: val, y: 0 }),
            Some('D') => Ok(Point { x: 0, y: -val }),
            Some('L') => Ok(Point { x: -val, y: 0 }),
            _ => Err(()),
        },
        _ => Err(()),
    }
}

fn expand_bound_rect(
    (left, bottom, width, height): (i32, i32, i32, i32),
    p: &Point,
) -> (i32, i32, i32, i32) {
    return (
        left.min(p.x),
        bottom.min(p.y),
        width.max(p.x - left + 1),
        height.max(p.y - bottom + 1),
    );
}

fn man_norm(x: i32, y: i32) -> i32 {
    x.abs() + y.abs()
}

fn main() {
    let wires: Vec<Vec<Point>> = lines_from_stdin!()
        .take(2)
        .map(|line| {
            let mut vector = vec![Point { x: 0, y: 0 }];
            line.split(',')
                .map(parse_point)
                .map(or_abort!("Failed to parse"))
                .for_each(|delta| {
                    let prev = vector.last().unwrap();
                    let next = Point {
                        x: prev.x + delta.x,
                        y: prev.y + delta.y,
                    };
                    vector.push(next);
                });
            vector
        })
        .collect();

    let rect = wires
        .iter()
        .flat_map(|w| w.iter())
        .fold((0, 0, 0, 0), expand_bound_rect);

    match util::part_id_from_cli() {
        util::PartID::One => {
            let mut map = map!(New rect, Fill 0);
            let mut min_dist = std::i32::MAX;
            for wire in wires {
                min_dist = std::i32::MAX;
                for (next, curr) in wire.iter().skip(1).zip(wire.iter()) {
                    line_map!(curr => next, |p: Point| {
                        if map!(map, p) != 0 {
                            min_dist = min_dist.min(man_norm(p.x, p.y));
                        } else {
                            map!(map, p, Set 1);
                        }
                    });
                }
            }
            println!("{}", min_dist);
        }
        util::PartID::Two => {
            let mut map = map!(New rect, Fill -1);
            let mut step_counter = 0;
            for (next, curr) in wires[0].iter().skip(1).zip(wires[0].iter()) {
                line_map!(curr => next, |p: Point| {
                    step_counter += 1;
                    if map!(map, p) == -1 {
                        map!(map, p, Set step_counter);
                    }
                });
            }
            step_counter = 0;
            let mut min_dist = std::i32::MAX;
            for (next, curr) in wires[1].iter().skip(1).zip(wires[1].iter()) {
                line_map!(curr => next, |p: Point| {
                    step_counter += 1;
                    if map!(map, p) > 0 {
                        min_dist = min_dist.min(map!(map, p) + step_counter);
                    }
                });
            }

            println!("{}", min_dist);
        }
    }
}
