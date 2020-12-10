struct PasswordRule {
    min: usize,
    max: usize,
    target: char,
    password: String,
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<PasswordRule> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");

            let mut min_max = parts.next().unwrap().split("-");
            let min = min_max.next().unwrap().parse().unwrap();
            let max = min_max.next().unwrap().parse().unwrap();

            let target = parts.next().unwrap().chars().next().unwrap();
            let password = parts.next().unwrap().to_string();

            PasswordRule {
                min,
                max,
                target,
                password,
            }
        }).collect()
}

#[aoc(day2, part1)]
fn solve_part1(input: &[PasswordRule]) -> usize {
    input
        .iter()
        .filter(|&p| {
            let target_count = p.password.chars().into_iter().filter(|c| c == &p.target).count();
            p.min <= target_count && target_count <= p.max
        })
        .count()
}

#[aoc(day2, part2)]
fn solve_part2(input: &[PasswordRule]) -> usize {
    input
        .iter()
        .filter(|&p| {
            let mut chars = p.password.chars().skip(p.min - 1);
            let first_match = chars.next().unwrap() == p.target;
            let second_match = chars.skip(p.max - p.min - 1).next().unwrap() == p.target;
            first_match ^ second_match
        })
        .count()
}
