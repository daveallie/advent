use std::collections::HashMap;

#[aoc(day15, part1)]
fn solve_part1(input: &str) -> usize {
    run(input, 2020)
}

#[aoc(day15, part2)]
fn solve_part2(input: &str) -> usize {
    run(input, 30_000_000)
}

fn run(input: &str, turn_limit: usize) -> usize {
    let mut turn_spoken: HashMap<usize, usize> = HashMap::new();
    let start_nums = input.split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    for (i, &num) in start_nums.iter().enumerate().rev().skip(1) {
        turn_spoken.insert(num, i + 1);
    }

    let mut turn = start_nums.len() + 1;
    let mut last_spoken = start_nums[start_nums.len() - 1];

    while turn <= turn_limit {
        let next_spoken = turn_spoken.get(&last_spoken).map(|ts| turn - 1 - ts).unwrap_or(0);
        turn_spoken.insert(last_spoken, turn - 1);

        last_spoken = next_spoken;
        turn += 1;
    }

    last_spoken
}
