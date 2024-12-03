pub fn part1(input: &str) -> u64 {
    let mut pass_rule = 0;
    for line in input.lines() {
        let report: Vec<i64> = line
            .split_whitespace()
            .map(|v| v.parse::<i64>().unwrap())
            .collect();
        if is_rule_safe(&report) {
            pass_rule += 1
        }
    }
    pass_rule
}
pub fn part2(input: &str) -> i64 {
    let mut pass_rule = 0;
    for line in input.lines() {
        let report: Vec<i64> = line
            .split_whitespace()
            .map(|v| v.parse::<i64>().unwrap())
            .collect();
        if is_rule_safe(&report) {
            pass_rule += 1;
            continue;
        }
        for i in 0..report.len() {
            let mut damp_report = report.clone();
            damp_report.remove(i);
            if is_rule_safe(&damp_report) {
                pass_rule += 1;
                break;
            }
        }
    }
    pass_rule
}

fn is_rule_safe(rule: &[i64]) -> bool {
    let mut previous = rule[0];
    let mut sort_order = None;
    for n in rule[1..].iter() {
        if sort_order.is_none() {
            match n.cmp(&previous) {
                std::cmp::Ordering::Less => {
                    sort_order = Some(SortOrder::Desc);
                }
                std::cmp::Ordering::Equal => return false,
                std::cmp::Ordering::Greater => {
                    sort_order = Some(SortOrder::Asc);
                }
            }
        }
        let order = sort_order.as_ref().unwrap();
        match order {
            SortOrder::Asc => {
                if n - previous > 3 || n - previous <= 0 {
                    return false;
                }
            }
            SortOrder::Desc => {
                if previous - n > 3 || previous - n <= 0 {
                    return false;
                }
            }
        }
        previous = *n;
    }
    true
}

enum SortOrder {
    Asc,
    Desc,
}
