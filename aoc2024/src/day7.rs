use winnow::{
    ascii::dec_uint,
    combinator::{opt, separated},
    stream::AsChar,
    token::take_while,
    PResult, Parser,
};

pub fn part1(input: &str) -> u64 {
    let mut input = input;
    let mut rs = 0;
    while !input.is_empty() {
        let (target, left) = parse_single_line(&mut input).unwrap();
        let (first, rest) = left.split_first().unwrap();
        if is_target_posible(target, first.parse().unwrap(), rest) {
            rs += target;
        }
    }
    rs
}
pub fn part2(input: &str) -> u64 {
    let mut input = input;
    let mut rs = 0;
    while !input.is_empty() {
        let (target, left) = parse_single_line(&mut input).unwrap();
        let (first, rest) = left.split_first().unwrap();
        if is_target_posible_2(target, first.parse().unwrap(), rest) {
            rs += target;
        }
    }
    rs
}

fn is_target_posible(target: u64, current: u64, left: &[&str]) -> bool {
    if current == target && left.is_empty() {
        return true;
    }
    if current > target || left.is_empty() {
        return false;
    }
    let (next, rest) = left.split_first().unwrap();
    let next_value = next.parse::<u64>().unwrap();
    is_target_posible(target, current * next_value, rest)
        || is_target_posible(target, current + next_value, rest)
}

fn is_target_posible_2(target: u64, current: u64, left: &[&str]) -> bool {
    if current == target && left.is_empty() {
        return true;
    }
    if current > target || left.is_empty() {
        return false;
    }
    let (next, rest) = left.split_first().unwrap();
    let next_value = next.parse::<u64>().unwrap();
    let pow = u64::pow(10, next.len() as u32);

    is_target_posible_2(target, current * next_value, rest)
        || is_target_posible_2(target, current + next_value, rest)
        || is_target_posible_2(target, (current * pow) + next_value, rest)
}

fn parse_single_line<'a>(input: &mut &'a str) -> PResult<(u64, Vec<&'a str>)> {
    let expected_output = dec_uint::<_, u64, _>(input)?;
    ": ".parse_next(input)?;
    let vec_str = separated(1.., take_while(1.., AsChar::is_dec_digit), " ").parse_next(input)?;
    let _ = opt("\n").parse_next(input)?;
    Ok((expected_output, vec_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_parse_line() {
        let input = "3267: 81 40 27";
        let mut xinput = input;
        let output = parse_single_line(&mut xinput).unwrap();
        assert_eq!(output.0, 3267);
        assert_eq!(output.1, ["81", "40", "27"]);
    }
}
