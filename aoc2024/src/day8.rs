use std::collections::HashSet;

use crate::grid::{index_to_coord, Coord, MapConfig};

pub fn part1(input: &str) -> u64 {
    let (map, map_config) = parse_map(input);
    let mut antinodes = HashSet::new();
    for (idx, &ch) in map.iter().enumerate() {
        if ch == "." {
            continue;
        }
        let all_index = find_all_except(&map, &ch, idx);
        let this_coord = index_to_coord(idx, &map_config);
        for target_idx in all_index {
            let target_coord = index_to_coord(target_idx, &map_config);
            let new_x = 2 * target_coord.0 - this_coord.0;
            let new_y = 2 * target_coord.1 - this_coord.1;
            let anti_coord = (new_x, new_y);
            if is_in_map(&anti_coord, &map_config) {
                antinodes.insert(anti_coord);
            }
        }
    }
    antinodes.len() as u64
}
pub fn part2(input: &str) -> u64 {
    let (map, map_config) = parse_map(input);
    let mut antinodes = HashSet::new();
    for (idx, &ch) in map.iter().enumerate() {
        if ch == "." {
            continue;
        }
        let all_index = find_all_except(&map, &ch, idx);
        let this_coord = index_to_coord(idx, &map_config);
        for target_idx in all_index {
            for i in 0.. {
                let target_coord = index_to_coord(target_idx, &map_config);
                let new_x = i * (target_coord.0 - this_coord.0) + target_coord.0;
                let new_y = i * (target_coord.1 - this_coord.1) + target_coord.1;
                let anti_coord = (new_x, new_y);
                if is_in_map(&anti_coord, &map_config) {
                    antinodes.insert(anti_coord);
                } else {
                    break;
                }
            }
        }
    }
    antinodes.len() as u64
}

fn parse_map(input: &str) -> (Vec<&str>, MapConfig) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut map = Vec::with_capacity(input.len());
    for c in input
        .lines()
        .flat_map(|line| line.split_inclusive(|_| true))
    {
        map.push(c);
    }
    (map, MapConfig { width, height })
}

fn find_all_except<T: PartialEq>(input: &[T], target: &T, except: usize) -> Vec<usize> {
    let mut rs = vec![];
    for (i, item) in input.iter().enumerate() {
        if item == target && i != except {
            rs.push(i);
        }
    }
    rs
}

fn is_in_map(coord: &Coord, map_config: &MapConfig) -> bool {
    if coord.0 < 0 || coord.1 < 0 {
        return false;
    }
    if coord.0 >= map_config.width as i64 || coord.1 >= map_config.height as i64 {
        return false;
    }
    true
}
