use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::{tag, take},
    character::complete::{multispace1, one_of, u64},
    multi::separated_list1,
    IResult, Parser,
};

#[derive(Debug)]
pub struct Path {
    direction: Direction,
    distance: u64,
}

impl Path {
    pub fn plot(&self, coord: &(i64, i64), map: &mut HashSet<(i64, i64)>) -> (i64, i64) {
        use Direction::*;
        let mut x = coord.0;
        let mut y = coord.1;
        let delta_x = match self.direction {
            Left => -1,
            Right => 1,
            _ => 0,
        };
        let delta_y = match self.direction {
            Up => 1,
            Down => -1,
            _ => 0,
        };
        for _ in 0..self.distance {
            x += delta_x;
            y += delta_y;
            map.insert((x, y));
        }
        (x, y)
    }
}
#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn parse_wires(input: &str) -> IResult<&str, (Vec<Path>, Vec<Path>)> {
    let (input, wire1) = separated_list1(tag(","), parse_path)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, wire2) = separated_list1(tag(","), parse_path)(input)?;
    Ok((input, (wire1, wire2)))
}

fn parse_path(input: &str) -> IResult<&str, Path> {
    use Direction::*;
    let (input, dir) = one_of("UDLR")
        .map(|c: char| match c {
            'U' => Up,
            'D' => Down,
            'L' => Left,
            'R' => Right,
            _ => unreachable!(),
        })
        .parse(input)?;
    let (input, distance) = u64(input)?;
    Ok((
        input,
        Path {
            direction: dir,
            distance,
        },
    ))
}
