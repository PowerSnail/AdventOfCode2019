mod util;

use std::collections::HashMap;
use std::io::BufRead;
use util::{error_exit, part_id_from_cli, PartID};

const TENS: [u32; 8] = [0, 1, 10, 100, 1000, 10000, 100000, 100000];
type Cache = HashMap<(u32, u32, bool), usize>;

macro_rules! digit {
    ($n:expr, $i:expr) => {
        $n / TENS[$i] % 10
    };
}

fn count(lo: u32, hi: u32, n_digit: usize, require_dup: bool, cache: &mut Cache) -> usize {
    if hi < lo {
        println!("count {} => {} = {}", lo, hi, 0);

        return 0;
    }
    if n_digit == 1 {
        let result = match require_dup {
            true => 0,
            false => (hi + 1 - lo) as usize,
        };
        println!("count {} => {} = {}", lo, hi, result);
        return result;
    }
    if cache.contains_key(&(lo, hi, require_dup)) {
        let result = cache[&(lo, hi, require_dup)];
        println!("count {} => {} = {}", lo, hi, result);
        return result;
    }

    let lo_digit = digit!(lo, n_digit);
    let hi_digit = digit!(hi, n_digit);

    let result = (lo_digit..=hi_digit)
        .map(|cur_d| {
            let lo = match cur_d == lo_digit {
                true => lo % TENS[n_digit],
                false => 0,
            };
            let hi = match cur_d == hi_digit {
                true => hi % TENS[n_digit],
                false => TENS[n_digit] - 1,
            };

            // next_d != cur_d
            let count1 = count(
                lo.max((cur_d + 1) * TENS[n_digit - 1]),
                hi.min(TENS[n_digit] - 1),
                n_digit - 1,
                require_dup,
                cache,
            );

            // next_d == cur_d
            let count2 = count(
                lo.max(cur_d * TENS[n_digit - 1]),
                hi.min((cur_d + 1) * TENS[n_digit - 1] - 1),
                n_digit - 1,
                false,
                cache,
            );

            count1 + count2
        })
        .sum();

    cache.insert((lo, hi, require_dup), result);
    println!("count {} => {} = {}", lo, hi, result);

    result
}

fn main() {
    let (lo, hi) = match lines_from_stdin!()
        .take(2)
        .map(|line| line.parse())
        .map(or_abort!("Failed to parse"))
        .collect::<Vec<u32>>()[..]
    {
        [lo, hi] => (lo, hi),
        _ => unreachable!(),
    };

    let mut cache: Cache = HashMap::new();
    let result = count(lo, hi, 6, true, &mut cache);

    println!("{}", result);
}
