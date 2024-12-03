
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u64,
    combinator::{opt, value},
    IResult,
};

pub fn parse_mul(input: &str) -> IResult<&str, Option<u64>> {
    let (input, Some(_)) = opt(tag("mul("))(input)? else {
        return Ok((&input[1..], None));
    };
    let (input, Some(u1)) = opt(u64)(input)? else {
        return Ok((input, None));
    };
    let (input, Some(_)) = opt(tag(","))(input)? else {
        return Ok((input, None));
    };
    let (input, Some(u2)) = opt(u64)(input)? else {
        return Ok((input, None));
    };
    let (input, Some(_)) = opt(tag(")"))(input)? else {
        return Ok((input, None));
    };
    Ok((input, Some(u1 * u2)))
}

pub fn parse_do_or_dont(input: &str) -> IResult<&str, Option<bool>> {
    let (input, Some(_)) = opt(tag("do"))(input)? else {
        return Ok((input, None));
    };
    let (input, Some(result)) =
        opt(alt((value(false, tag("n't()")), value(true, tag("()")))))(input)?
    else {
        return Ok((input, None));
    };
    Ok((input, Some(result)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_parse_mul() {
        let input = "mul(2,4)";
        assert_eq!(parse_mul(input).unwrap().1, Some(8));
    }
}
