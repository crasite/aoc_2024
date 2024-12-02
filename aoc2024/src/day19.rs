mod parser;
use std::collections::HashMap;

use anyhow::anyhow;
use parser::{parse_rules, parse_xmas_input};

pub fn part1(input: &str) -> u64 {
    let mut rules = HashMap::new();
    let mut xmas_part_list = vec![];
    let mut line_iter = input.lines();
    for line in line_iter.by_ref() {
        if line.is_empty() {
            break;
        }
        let rule = Rules::new(line).unwrap();
        rules.insert(rule.name, rule);
    }
    for line in line_iter {
        let part = XmasPart::new(line).unwrap();
        xmas_part_list.push(part);
    }

    let mut total = 0;
    for part in xmas_part_list {
        let mut target = rules.get("in").unwrap().get_next_target(&part);
        while let Target::Rule(r) = target {
            target = rules.get(r).unwrap().get_next_target(&part);
        }
        if let Target::Approve = target {
            total += part.score();
        }
    }
    total
}

pub fn part2(input: &str) -> u64 {
    let mut rules = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let rule = Rules::new(line).unwrap();
        rules.insert(rule.name, rule);
    }
    let in_rule = rules.get("in").unwrap();
    get_set_score(&rules, &XmasRange::default(), in_rule)
}

fn get_set_score(rule_set: &HashMap<&str, Rules>, range: &XmasRange, rule: &Rules) -> u64 {
    let mut total = 0;
    let mut current_range = range.clone();
    for cond in rule.conditions.iter() {
        if let RuleType::Immediate = cond.rule_type {
            match cond.target {
                Target::Rule(r) => {
                    let pass_rule = rule_set.get(r).unwrap();
                    total += get_set_score(rule_set, &current_range, pass_rule);
                }
                Target::Reject => {}
                Target::Approve => {
                    total += current_range.total();
                }
            }
            continue;
        }
        match cond.target {
            Target::Rule(r) => {
                let (pass_range, next_range) = current_range.split(cond);
                let pass_rule = rule_set.get(r).unwrap();
                total += get_set_score(rule_set, &pass_range, pass_rule);
                current_range = next_range;
            }
            Target::Reject => {
                let (_, next_range) = current_range.split(cond);
                current_range = next_range
            }
            Target::Approve => {
                let (approve_range, next_range) = current_range.split(cond);
                total += approve_range.total();
                current_range = next_range
            }
        }
    }
    total
}

#[derive(Clone, Debug)]
struct XmasRange {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

impl XmasRange {
    fn total(&self) -> u64 {
        (1 + self.x.1 - self.x.0)
            * (1 + self.m.1 - self.m.0)
            * (1 + self.a.1 - self.a.0)
            * (1 + self.s.1 - self.s.0)
    }
    fn split(&self, cond: &Condition) -> (Self, Self) {
        let (matched_range, unmatched_range) = match cond.rule_type {
            RuleType::LessThan => ((1, cond.amount - 1), (cond.amount, 4000)),
            RuleType::GreaterThan => ((cond.amount + 1, 4000), (1, cond.amount)),
            _ => {
                unreachable!()
            }
        };
        match cond.part {
            Part::X => (
                Self {
                    x: intersect_range(self.x, matched_range),
                    ..self.clone()
                },
                Self {
                    x: intersect_range(self.x, unmatched_range),
                    ..self.clone()
                },
            ),
            Part::M => (
                Self {
                    m: intersect_range(self.m, matched_range),
                    ..self.clone()
                },
                Self {
                    m: intersect_range(self.m, unmatched_range),
                    ..self.clone()
                },
            ),
            Part::A => (
                Self {
                    a: intersect_range(self.a, matched_range),
                    ..self.clone()
                },
                Self {
                    a: intersect_range(self.a, unmatched_range),
                    ..self.clone()
                },
            ),
            Part::S => (
                Self {
                    s: intersect_range(self.s, matched_range),
                    ..self.clone()
                },
                Self {
                    s: intersect_range(self.s, unmatched_range),
                    ..self.clone()
                },
            ),
            _ => unreachable!(),
        }
    }
}

impl Default for XmasRange {
    fn default() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rules<'a> {
    name: &'a str,
    conditions: Vec<Condition<'a>>,
}

impl<'a> Rules<'a> {
    fn new(input: &'a str) -> anyhow::Result<Rules<'a>> {
        let (_, rule) = parse_rules(input).map_err(|e| anyhow!("unable to map: {:?}", e))?;
        Ok(rule)
    }

    fn get_next_target(&self, part: &XmasPart) -> &Target {
        for cond in self.conditions.iter() {
            match cond.part {
                Part::X => {
                    if is_cond_pass(&cond.rule_type, cond.amount, part.x) {
                        return &cond.target;
                    }
                }
                Part::M => {
                    if is_cond_pass(&cond.rule_type, cond.amount, part.m) {
                        return &cond.target;
                    }
                }
                Part::A => {
                    if is_cond_pass(&cond.rule_type, cond.amount, part.a) {
                        return &cond.target;
                    }
                }
                Part::S => {
                    if is_cond_pass(&cond.rule_type, cond.amount, part.s) {
                        return &cond.target;
                    }
                }
                Part::Any => {
                    return &cond.target;
                }
            }
        }
        unreachable!("should not be able to reach")
    }
}

#[derive(Debug, PartialEq, Eq)]
struct XmasPart {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl XmasPart {
    fn new(input: &str) -> anyhow::Result<Self> {
        let (_, rs) = parse_xmas_input(input).map_err(|e| anyhow!("error: {:?}", e))?;
        Ok(rs)
    }

    fn score(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Condition<'a> {
    part: Part,
    rule_type: RuleType,
    amount: u64,
    target: Target<'a>,
}

#[derive(Debug, PartialEq, Eq)]
enum Part {
    X,
    M,
    A,
    S,
    Any,
}

#[derive(Debug, PartialEq, Eq)]
enum RuleType {
    LessThan,
    GreaterThan,
    Immediate,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Target<'a> {
    Rule(&'a str),
    Reject,
    Approve,
}

fn is_cond_pass(rt: &RuleType, threshold: u64, check: u64) -> bool {
    use RuleType::*;
    match rt {
        LessThan => check < threshold,
        GreaterThan => check > threshold,
        Immediate => true,
    }
}

fn intersect_range(r1: (u64, u64), r2: (u64, u64)) -> (u64, u64) {
    (r1.0.max(r2.0), r1.1.min(r2.1))
}

#[cfg(test)]
mod tests {
    use parser::parse_condition;

    use super::*;

    #[test]
    fn could_parse_rules() {
        let sample = "px{a<2006:qkq,m>2090:A,rfg}";
        let expect = Rules {
            name: "px",
            conditions: vec![
                Condition {
                    target: Target::Rule("qkq"),
                    part: Part::A,
                    rule_type: RuleType::LessThan,
                    amount: 2006,
                },
                Condition {
                    target: Target::Approve,
                    part: Part::M,
                    rule_type: RuleType::GreaterThan,
                    amount: 2090,
                },
                Condition {
                    target: Target::Rule("rfg"),
                    part: Part::Any,
                    rule_type: RuleType::Immediate,
                    amount: 0,
                },
            ],
        };
        let rules = Rules::new(sample).expect("unable to parse");
        assert_eq!(rules, expect);
    }

    #[test]
    fn could_parse_condition() {
        let sample = "a<2006:qkq,m>2090:A,rfg}";
        let expect = Condition {
            target: Target::Rule("qkq"),
            part: Part::A,
            rule_type: RuleType::LessThan,
            amount: 2006,
        };
        let cond = parse_condition(sample).unwrap().1;
        assert_eq!(cond, expect);
    }

    #[test]
    fn could_parse_xmas() {
        let input = "{x=787,m=2655,a=1222,s=2876}";
        let expected = XmasPart {
            x: 787,
            m: 2655,
            a: 1222,
            s: 2876,
        };
        let rs = XmasPart::new(input).unwrap();
        assert_eq!(expected, rs)
    }
}
