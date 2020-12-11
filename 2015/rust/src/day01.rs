#[aoc(day1, part1)]
fn solve_part1(input: &str) -> i32 {
    input.chars()
        .fold(0, |acc, c| acc + (if c == '(' { 1 } else { -1 }))
}

#[aoc(day1, part2)]
fn solve_part2(input: &str) -> usize {
    let mut floor = 0;

    for (i, char) in input.chars().enumerate() {
        floor += if char == '(' { 1 } else { -1 };

        if floor < 0 {
            return i + 1
        }
    }

    0
}
