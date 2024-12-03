use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use parser::parse_wires;

mod parser;
pub fn part1(input: &str) -> u64 {
    let mut rs = u64::MAX;
    let mut plot = HashSet::new();
    let mut plot2 = HashSet::new();
    let (wire1, wire2) = parse_wires(input).unwrap().1;
    let mut coord = (0, 0);
    for path in wire1 {
        coord = path.plot(&coord, &mut plot);
    }
    coord = (0, 0);
    for path in wire2 {
        coord = path.plot(&coord, &mut plot2);
    }
    for (x, y) in plot.intersection(&plot2) {
        rs = rs.min(x.abs() as u64 + y.abs() as u64);
    }
    rs
}

pub fn part2(input: &str) -> u64 {
    todo!()
}
