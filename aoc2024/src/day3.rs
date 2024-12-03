use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use parser::{parse_do_or_dont, parse_mul};

mod parser;
pub fn part1(input: &str) -> u64 {
    let mut input = input;
    let mut rs = 0;
    while input.len() >= 8 {
        if let Ok((new_input, v)) = parse_mul(input) {
            input = new_input;
            if let Some(v) = v {
                rs += v;
            }
        } else {
            unreachable!()
        }
    }
    rs
}

pub fn part2(input: &str) -> u64 {
    let mut input = input;
    let mut rs = 0;
    let mut enabled = true;
    while input.len() >= 8 {
        if let Ok((new_input, rs)) = parse_do_or_dont(input) {
            input = new_input;
            if let Some(enable) = rs {
                enabled = enable;
                continue;
            }
        }
        if let Ok((new_input, v)) = parse_mul(input) {
            input = new_input;
            let Some(v) = v else { continue };
            if enabled {
                rs += v;
            }
            continue;
        }
    }
    rs
}
