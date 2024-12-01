use std::collections::HashMap;

pub fn part1(input: &str) -> u64 {
    let mut list_a = vec![];
    let mut list_b = vec![];
    for line in input.lines() {
        let mut white_split = line.split_whitespace();
        let input_a = white_split.next().unwrap();
        let input_b = white_split.next().unwrap();
        list_a.push(input_a.parse::<u64>().unwrap());
        list_b.push(input_b.parse::<u64>().unwrap());
    }
    list_a.sort();
    list_b.sort();
    list_a.iter().zip(list_b).map(|(a, b)| a.abs_diff(b)).sum()
}
pub fn part2(input: &str) -> u64 {
    let mut list_a = vec![];
    let mut list_b: HashMap<u64, u64> = HashMap::new();
    for line in input.lines() {
        let mut white_split = line.split_whitespace();
        let input_a = white_split.next().unwrap();
        let input_b = white_split.next().unwrap().parse::<u64>().unwrap();
        list_a.push(input_a.parse::<u64>().unwrap());
        if let Some(v) = list_b.get(&input_b) {
            list_b.insert(input_b, v + 1);
        } else {
            list_b.insert(input_b, 1);
        }
    }
    let mut result = 0;
    for n in list_a {
        if let Some(v) = list_b.get(&n) {
            result += n * v;
        }
    }
    result
}
