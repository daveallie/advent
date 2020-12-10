use std::collections::HashSet;

#[aoc(day6, part1)]
fn solve_part1(input: &str) -> usize {
    input.split("\n\n")
        .map(|group| {
            group.chars().filter(|c| c.is_alphabetic()).collect::<HashSet<char>>().len()
        }).sum()
}

#[aoc(day6, part2)]
fn solve_part2(input: &str) -> usize {
    input.split("\n\n")
        .map(|group| {
            let mut lines = group.lines();
            let all_answered: HashSet<char> = lines.next().unwrap().chars().collect();

            lines.fold(all_answered, |acc, line|
                acc.intersection(&line.chars().collect::<HashSet<char>>()).cloned().collect()
            ).len()
        }).sum()
}
