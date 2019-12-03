mod util;

use std::io::BufRead;
use util::error_exit;

struct Point {
    x: i32,
    y: i32,
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

fn in_range(v: i32, (lo, hi): (i32, i32)) -> bool {
    lo <= v && v <= hi
}

fn wire_range(beg: i32, delta: i32) -> (i32, i32) {
    let lo = std::cmp::min(beg, beg + delta);
    let hi = std::cmp::max(beg, beg + delta);
    (lo + 1, hi)
}

fn range_intersection((beg1, end1): (i32, i32), (beg2, end2): (i32, i32)) -> (i32, i32) {
    (std::cmp::max(beg1, beg2), std::cmp::min(end1, end2))
}

fn min_abs((beg, end): (i32, i32)) -> Option<i32> {
    if end < beg {
        None
    } else if beg * end <= 0 {
        Some(0)
    } else {
        Some(std::cmp::min(beg.abs(), end.abs()))
    }
}

fn interset_perpendicular(
    vert_o: &Point,
    vert_v: &Point,
    hori_o: &Point,
    hori_v: &Point,
) -> Option<i32> {
    if in_range(vert_o.x, wire_range(hori_o.x, hori_v.x))
        && in_range(hori_o.y, wire_range(vert_o.y, vert_v.y))
    {
        println!("Found inter {}, {}", vert_o.x, hori_o.y);
        Some(man_norm(vert_o.x, hori_o.y))
    } else {
        None
    }
}

fn intersect_norm(o1: &Point, v1: &Point, o2: &Point, v2: &Point) -> Option<i32> {
    match (v1, v2) {
        (Point { x: 0, y: _ }, Point { x: _, y: 0 }) => interset_perpendicular(&o1, &v1, &o2, &v2),
        (Point { x: _, y: 0 }, Point { x: 0, y: _ }) => interset_perpendicular(&o2, &v2, &o1, &v1),
        (Point { x: 0, y: _ }, Point { x: 0, y: _ }) if o1.x == o2.x => {
            match min_abs(range_intersection(
                wire_range(o1.y, v1.y),
                wire_range(o2.y, v2.y),
            )) {
                Some(y) => {
                    Some(man_norm(o1.x, y))
                }
                _ => None,
            }
        }
        (Point { x: _, y: 0 }, Point { x: _, y: 0 }) if o1.y == o2.y => {
            match min_abs(range_intersection(
                wire_range(o1.x, v1.x),
                wire_range(o2.x, v2.x),
            )) {
                Some(x) => {
                    println!("Found intersectin {},{}", x, o1.y);
                    Some(man_norm(x, o1.y))
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn man_norm(x: i32, y: i32) -> i32 {
    x.abs() + y.abs()
}

fn main() {
    let inputs: Vec<Vec<Point>> = lines_from_stdin!()
        .map(|line| {
            line.split(',')
                .map(parse_point)
                .map(or_abort!("Failed to parse"))
                .collect()
        })
        .collect();

    let w1 = &inputs[0];
    let w2 = &inputs[1];

    let mut min_dist: i32 = 999999;
    let mut o1 = Point { x: 0, y: 0 };
    for v1 in w1 {
        let mut o2 = Point { x: 0, y: 0 };
        for v2 in w2 {
            match intersect_norm(&o1, v1, &o2, v2) {
                Some(dist) => min_dist = min_dist.min(dist),
                None => (),
            }
            o2.x += v2.x;
            o2.y += v2.y;
        }
        o1.x += v1.x;
        o1.y += v1.y;
    }

    println!("{}", min_dist);
}
