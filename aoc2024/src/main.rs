#![allow(unused_imports)]
use std::time::{Duration, Instant};

use aoc2024::{
    challenge::{DAY6CHALLENGE, DAY6TEST},
    day6::{part1, part2},
};

fn main() {
    let now = Instant::now();
    let rs = part1(DAY6CHALLENGE);
    println!(
        "part 1 result is: {} in {}",
        rs,
        duration_to_string(now.elapsed())
    );
    let now = Instant::now();
    let rs = part2(DAY6CHALLENGE);
    println!(
        "part 2 result is: {} in {}",
        rs,
        duration_to_string(now.elapsed())
    );
}

fn duration_to_string(dur: Duration) -> String {
    if dur.as_secs() > 10 {
        format!("{} seconds", dur.as_secs())
    } else if dur.as_millis() > 1 {
        format!("{} ms", dur.as_millis())
    } else if dur.as_micros() > 10 {
        format!("{} μs", dur.as_micros())
    } else {
        format!("{} ns", dur.as_nanos())
    }
}
