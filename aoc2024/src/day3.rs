use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use parser::{parse_do, parse_dont, parse_mul};

mod parser;
pub fn part1(input: &str) -> u64 {
    let mut input = input;
    let mut rs = 0;
    while input.len() >= 8 {
        if let Ok((new_input, v)) = parse_mul(input) {
            input = new_input;
            rs += v;
        } else {
            input = &input[1..];
        }
    }
    rs
}

pub fn part2(input: &str) -> u64 {
    let mut input = input;
    let mut rs = 0;
    let mut enabled = true;
    while input.len() >= 8 {
        if let Ok((new_input, v)) = parse_mul(input) {
            input = new_input;
            if enabled {
                rs += v;
            }
            continue;
        }
        if let Ok((new_input, _)) = parse_do(input) {
            input = new_input;
            enabled = true;
            continue;
        }
        if let Ok((new_input, _)) = parse_dont(input) {
            input = new_input;
            enabled = false;
            continue;
        }
        input = &input[1..];
    }
    rs
}
