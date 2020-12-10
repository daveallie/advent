use std::collections::HashSet;

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc(day1, part1, loops)]
fn solve_part1_loops(input: &[usize]) -> usize {
    for (i, num1) in input.iter().enumerate() {
        for num2 in input[i..].iter() {
            if num1 + num2 == 2020 {
                return num1 * num2
            }
        }
    }

    0
}

#[aoc(day1, part1, set)]
fn solve_part1_set(input: &[usize]) -> usize {
    let mut looking = HashSet::new();
    for num in input {
        if looking.contains(num) {
            return (2020 - num) * num
        }
        looking.insert(2020 - num);
    }
    0
}

#[aoc(day1, part2)]
fn solve_part2(input: &[usize]) -> usize {
    for (i, num1) in input.iter().enumerate() {
        for (j, num2) in input[i..].iter().enumerate() {
            for num3 in input[i+j..].iter() {
                if num1 + num2 + num3 == 2020 {
                    return num1 * num2 * num3
                }
            }
        }
    }

    0
}
