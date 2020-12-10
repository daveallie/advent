#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Vec<bool>> {
    input.lines().map(|line|
        line.chars().map(|c| c == '#').collect()
    ).collect()
}

#[aoc(day3, part1)]
fn solve_part1(grid: &[Vec<bool>]) -> usize {
    calculate(grid, 3, 1)
}

#[aoc(day3, part2)]
fn solve_part2(grid: &[Vec<bool>]) -> usize {
    calculate(grid, 1, 1) * calculate(grid, 3, 1) *
        calculate(grid, 5, 1) * calculate(grid, 7, 1) *
        calculate(grid, 1, 2)
}

fn calculate(grid: &[Vec<bool>], across: usize, down: usize) -> usize {
    grid.iter().step_by(down).enumerate()
        .filter(|(i, row)| *row.get((across * i) % row.len()).unwrap())
        .count()
}
