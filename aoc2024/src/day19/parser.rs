
use anyhow::anyhow;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until1, take_while},
    character::complete::u64 as parse_u64,
    combinator::{map_res, value},
    multi::separated_list1,
    AsChar, IResult, Parser,
};

use super::{Condition, Part, RuleType, Rules, Target, XmasPart};

pub(super) fn parse_xmas_input(input: &str) -> IResult<&str, XmasPart> {
    let (input, _) = tag("{x=")(input)?;
    let (input, x) = parse_u64(input)?;
    let (input, _) = tag(",m=")(input)?;
    let (input, m) = parse_u64(input)?;
    let (input, _) = tag(",a=")(input)?;
    let (input, a) = parse_u64(input)?;
    let (input, _) = tag(",s=")(input)?;
    let (input, s) = parse_u64(input)?;
    Ok((input, XmasPart { x, m, a, s }))
}

pub(super) fn parse_rules(input: &str) -> IResult<&str, Rules> {
    let (input, name) = take_until1("{")(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, conds) = parse_condition_list(input)?;
    Ok((
        input,
        Rules {
            name,
            conditions: conds,
        },
    ))
}
fn parse_condition_list(input: &str) -> IResult<&str, Vec<Condition>> {
    let (input, mut condition_list) = separated_list1(tag(","), parse_condition)(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, last_target) = parse_target(input)?;
    condition_list.push(Condition {
        target: last_target,
        amount: 0,
        rule_type: RuleType::Immediate,
        part: Part::Any,
    });
    Ok((input, condition_list))
}
pub(super) fn parse_condition(input: &str) -> IResult<&str, Condition> {
    let (input, part) = map_res(take(1usize), |c: &str| match c {
        "x" => Ok(Part::X),
        "m" => Ok(Part::M),
        "a" => Ok(Part::A),
        "s" => Ok(Part::S),
        _ => Err(anyhow!("non existing character")),
    })(input)?;
    let (input, rule_type) = map_res(take(1usize), |c: &str| match c {
        "<" => Ok(RuleType::LessThan),
        ">" => Ok(RuleType::GreaterThan),
        _ => Err(anyhow!("non existing ruletype")),
    })(input)?;
    let (input, count) = parse_u64(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, target) = parse_target(input)?;
    Ok((
        input,
        Condition {
            part,
            rule_type,
            amount: count,
            target,
        },
    ))
}

fn parse_target(input: &str) -> IResult<&str, Target> {
    alt((
        value(Target::Approve, tag("A")),
        value(Target::Reject, tag("R")),
        take_while(|c: char| c.is_alpha()).map(Target::Rule),
    ))(input)
}
