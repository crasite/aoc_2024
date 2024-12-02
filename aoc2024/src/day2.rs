use itertools::Itertools;

pub fn part1(input: &str) -> u64 {
    let mut program = vec![];
    for v in input.trim().split(',').map(|s| s.parse::<u64>().unwrap()) {
        program.push(v);
    }
    run_program(&program, 12, 2)
}
pub fn part2(input: &str) -> u64 {
    let mut program = vec![];
    let target = 19690720;
    for v in input.trim().split(',').map(|s| s.parse::<u64>().unwrap()) {
        program.push(v);
    }
    for (i1, i2) in (0..100).cartesian_product(0..100) {
        if run_program(&program, i1, i2) == target {
            return 100 * i1 + i2;
        }
    }
    unreachable!()
}

fn run_program(program: &[u64], i1: u64, i2: u64) -> u64 {
    let mut program = program.to_vec();
    let mut pointer = 0;
    program[1] = i1;
    program[2] = i2;
    while let Some((p, val)) = run_next_instruction(&program, pointer) {
        program[p] = val;
        pointer += 4;
    }
    program[0]
}

fn run_next_instruction(program: &[u64], pointer: usize) -> Option<(usize, u64)> {
    match program[pointer] {
        99 => None,
        1 => {
            let v1 = program[program[pointer + 1] as usize];
            let v2 = program[program[pointer + 2] as usize];
            let p = program[pointer + 3] as usize;
            Some((p, v1 + v2))
        }
        2 => {
            let v1 = program[program[pointer + 1] as usize];
            let v2 = program[program[pointer + 2] as usize];
            let p = program[pointer + 3] as usize;
            Some((p, v1 * v2))
        }
        _ => {
            unreachable!()
        }
    }
}
