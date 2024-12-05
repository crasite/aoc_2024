use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use winnow::{
    ascii::dec_uint,
    combinator::{repeat, separated},
    PResult, Parser,
};

pub fn part1(input: &str) -> u64 {
    let mut rs = 0;
    let mut input = input;
    let rules = parse_simple_rule_list(&mut input).unwrap();
    let combined_rules = CombinedRule::from_list(rules.into_iter());
    let update_list = parse_all_update(&mut input).unwrap();
    for list in update_list.iter() {
        if is_in_rule(&list.0, &combined_rules) {
            rs += list.0[list.0.len() / 2]
        };
    }
    rs
}

pub fn part2(input: &str) -> u64 {
    let mut rs = 0;
    let mut input = input;
    let rules = parse_simple_rule_list(&mut input).unwrap();
    let combined_rules = CombinedRule::from_list(rules.into_iter());
    let update_list = parse_all_update(&mut input).unwrap();
    for list in update_list.into_iter() {
        if is_in_rule(&list.0, &combined_rules) {
            continue;
        };
        let mut new_list: VecDeque<u64> = list.0.into();
        while let Some(i) = is_in_rule_reason(new_list.make_contiguous(), &combined_rules) {
            let pop_v = new_list.remove(i).unwrap();
            new_list.push_front(pop_v);
        }
        if !is_in_rule(new_list.as_slices().0, &combined_rules) {
            panic!("wow");
        }
        rs += new_list[new_list.len() / 2]
    }
    rs
}

struct SimpleRule {
    before: u64,
    after: u64,
}

struct CombinedRule {
    after: Vec<u64>,
}

impl CombinedRule {
    fn from_list(iter: impl Iterator<Item = SimpleRule>) -> HashMap<u64, CombinedRule> {
        let mut result = HashMap::new();
        for rule in iter {
            let Some(cr) = result.get_mut(&rule.before) else {
                result.insert(
                    rule.before,
                    CombinedRule {
                        after: vec![rule.after],
                    },
                );
                continue;
            };
            cr.after.push(rule.after);
        }
        result
    }
}

#[derive(Debug)]
struct Update(Vec<u64>);

fn parse_all_update(input: &mut &str) -> PResult<Vec<Update>> {
    "\n".parse_next(input)?;
    let rs = separated(1.., parse_update, "\n").parse_next(input)?;
    Ok(rs)
}

fn parse_update(input: &mut &str) -> PResult<Update> {
    let list = separated(1.., dec_uint::<_, u64, _>, ",").parse_next(input)?;
    Ok(Update(list))
}

fn parse_simple_rule_list(input: &mut &str) -> PResult<Vec<SimpleRule>> {
    let rs = repeat(1.., parse_simple_rule).parse_next(input)?;
    Ok(rs)
}
fn parse_simple_rule(input: &mut &str) -> PResult<SimpleRule> {
    let n1: u64 = dec_uint(input)?;
    "|".parse_next(input)?;
    let n2: u64 = dec_uint(input)?;
    "\n".parse_next(input)?;
    Ok(SimpleRule {
        before: n1,
        after: n2,
    })
}

fn is_in_rule(update: &[u64], rules: &HashMap<u64, CombinedRule>) -> bool {
    let mut passed_number = vec![];
    for n in update.iter() {
        let Some(check) = rules.get(n) else {
            passed_number.push(n);
            continue;
        };
        if check.after.iter().any(|v| passed_number.contains(&v)) {
            return false;
        }
        passed_number.push(n);
    }
    true
}
fn is_in_rule_reason(update: &[u64], rules: &HashMap<u64, CombinedRule>) -> Option<usize> {
    let mut passed_number = vec![];
    for (i, n) in update.iter().enumerate() {
        let Some(check) = rules.get(n) else {
            passed_number.push(n);
            continue;
        };
        for c in check.after.iter() {
            if passed_number.contains(&c) {
                return Some(i);
            }
        }
        passed_number.push(n);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn could_parse_simple_rule() {
        let mut input = "47|53\n";
        let rule = parse_simple_rule(&mut input).unwrap();
        assert_eq!(47, rule.before);
        assert_eq!(53, rule.after);
    }
    #[test]
    fn could_parse_simple_rule_list() {
        let mut input = "47|53\n22|33\n";
        let rules = parse_simple_rule_list(&mut input).unwrap();
        assert_eq!(47, rules[0].before);
        assert_eq!(53, rules[0].after);
        assert_eq!(22, rules[1].before);
        assert_eq!(33, rules[1].after);
    }

    #[test]
    fn could_parse_simple_combined() {
        let mut input = "47|53\n22|33\n47|34\n";
        let rules = parse_simple_rule_list(&mut input).unwrap();
        let combined_rule = CombinedRule::from_list(rules.into_iter());
        assert_eq!(combined_rule.get(&47).unwrap().after, vec![53, 34]);
    }
}
