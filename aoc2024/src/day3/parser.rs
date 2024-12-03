use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::{tag, take},
    character::complete::{multispace1, one_of, u64},
    multi::separated_list1,
    IResult, Parser,
};

pub fn parse_mul(input: &str) -> IResult<&str, u64> {
    let (input, _) = tag("mul(")(input)?;
    let (input, u1) = u64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, u2) = u64(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, u1 * u2))
}

pub fn parse_do(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, ()))
}
pub fn parse_dont(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_parse_mul() {
        let input = "mul(2,4)";
        assert_eq!(parse_mul(input).unwrap().1, 8);
    }
}
