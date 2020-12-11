#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<[usize; 3]> {
    input.lines()
        .map(|l| {
            let mut dims = l
                .split("x")
                .map(|p| p.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            dims.sort();
            [dims[0], dims[1], dims[2]]
        }).collect()
}

#[aoc(day2, part1)]
fn solve_part1(input: &[[usize; 3]]) -> usize {
    input.iter()
        .fold(0, |acc, dims| {
            acc +
                3 * dims[0] * dims[1] +
                2 * dims[0] * dims[2] +
                2 * dims[1] * dims[2]
        })
}

#[aoc(day2, part2)]
fn solve_part2(input: &[[usize; 3]]) -> usize {
    input.iter()
        .fold(0, |acc, dims| {
            acc +
                2 * (dims[0] + dims[1]) +
                dims[0] * dims[1] * dims[2]
        })
}
