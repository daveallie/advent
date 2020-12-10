#[aoc(day5, part1)]
fn solve_part1(input: &str) -> u32 {
    input.lines()
        .map(|line| get_seat_num(line))
        .max()
        .unwrap()
}

#[aoc(day5, part2)]
fn solve_part2(input: &str) -> u32 {
    let mut seats: Vec<u32> = input.lines()
        .map(|line| get_seat_num(line))
        .collect();
    seats.sort();

    seats.windows(2).find(|w| w[1] - w[0] > 1).unwrap()[0] + 1
}

fn get_seat_num(line: &str) -> u32 {
    line.chars().rev().enumerate()
        .map(|(i, c)| 2_u32.pow(i as u32) * (if c == 'B' || c == 'R' { 1 } else { 0 }))
        .sum::<u32>()
}
