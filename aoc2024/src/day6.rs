use std::collections::HashSet;

use crate::grid::{coord_to_index, get_at, index_to_coord, Coord, MapConfig};

pub fn part1(input: &str) -> u64 {
    let map_config = get_map_config(input);
    let (map, mut guard_pos) = parse_map(input, &map_config);
    let mut guard_direction = Direction::Up;
    let mut moved_path = HashSet::new();
    moved_path.insert(guard_pos);
    while let Some((direction, new_pos)) =
        guard_move(&map, &map_config, &guard_pos, &guard_direction)
    {
        moved_path.insert(new_pos);
        guard_pos = new_pos;
        guard_direction = direction;
    }
    moved_path.len() as u64
}

pub fn part2(input: &str) -> u64 {
    let map_config = get_map_config(input);
    let (map, original_guard_pos) = parse_map(input, &map_config);
    let mut guard_pos = original_guard_pos;
    let mut guard_direction = Direction::Up;
    let mut moved_path = HashSet::new();
    moved_path.insert(guard_pos);
    while let Some((direction, new_pos)) =
        guard_move(&map, &map_config, &guard_pos, &guard_direction)
    {
        moved_path.insert(new_pos);
        guard_pos = new_pos;
        guard_direction = direction;
    }

    let mut rs = 0;
    moved_path.remove(&original_guard_pos);
    for coord in moved_path.into_iter() {
        let mut new_map = map.clone();
        let i = coord_to_index(coord, &map_config);
        new_map[i] = Unit::Wall;
        if move_till_stuck(original_guard_pos, &new_map, &map_config) {
            rs += 1
        }
    }

    rs
}

fn move_till_stuck(guard_pos: Coord, map: &[Unit], map_config: &MapConfig) -> bool {
    let mut guard_pos = guard_pos;
    let mut guard_direction = Direction::Up;
    let mut moved_path = HashSet::new();
    moved_path.insert((guard_pos, guard_direction));
    while let Some((direction, new_pos)) = guard_move(map, map_config, &guard_pos, &guard_direction)
    {
        let is_old = !moved_path.insert((new_pos, direction));
        guard_pos = new_pos;
        guard_direction = direction;
        if is_old {
            return true;
        }
    }
    false
}

fn guard_move(
    map: &[Unit],
    map_config: &MapConfig,
    guard: &Coord,
    direction: &Direction,
) -> Option<(Direction, Coord)> {
    use Direction::*;
    use Unit::*;
    let (dx, dy) = match *direction {
        Up => (0, -1),
        Down => (0, 1),
        Left => (-1, 0),
        Right => (1, 0),
    };
    let next_pos = (guard.0 + dx, guard.1 + dy);
    let next_unit = get_at(&next_pos, map, map_config)?;
    match next_unit {
        Ground => Some((*direction, next_pos)),
        Wall => match *direction {
            Up => guard_move(map, map_config, guard, &Right),
            Down => guard_move(map, map_config, guard, &Left),
            Left => guard_move(map, map_config, guard, &Up),
            Right => guard_move(map, map_config, guard, &Down),
        },
    }
}

fn get_map_config(input: &str) -> MapConfig {
    let mut lines = input.lines();
    let width = lines.next().unwrap().trim().len();
    let height = lines.count() + 1;
    MapConfig { width, height }
}

fn parse_map(input: &str, map_config: &MapConfig) -> (Vec<Unit>, Coord) {
    use Unit::*;
    let mut rs = vec![];
    let mut guard_pos = None;
    for (i, c) in input.lines().flat_map(|l| l.chars()).enumerate() {
        match c {
            '.' => rs.push(Ground),
            '#' => rs.push(Wall),
            '^' => {
                rs.push(Ground);
                guard_pos = Some(index_to_coord(i, map_config));
            }
            _ => unreachable!(),
        }
    }
    (rs, guard_pos.unwrap())
}

#[derive(Debug, Copy, Clone)]
enum Unit {
    Ground,
    Wall,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
