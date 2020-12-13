#[aoc(day8, part1)]
fn solve_part1(input: &str) -> i32 {
    input.lines()
        .map(|line| {
            let mut code_points = 0;
            let mut skip_iters = 0;

            for i in 1..line.len() - 1 {
                if skip_iters > 0 {
                    skip_iters -= 1;
                    continue;
                }

                let c = line.chars().nth(i).unwrap();

                if c == '\\' {
                    let c2 = line.chars().nth(i + 1).unwrap();

                    if c2 == 'x' {
                        skip_iters = 3;
                    } else {
                        skip_iters = 1;
                    }
                }

                code_points += 1;
            }

            (line.len() as i32) - code_points
        })
        .sum()
}

#[aoc(day8, part2)]
fn solve_part2(input: &str) -> i32 {
    input.lines()
        .map(|line| {
            let mut new_len = 0;

            for c in line.chars() {
                if c == '\\' || c == '"' {
                    new_len += 1;
                }

                new_len += 1;
            }

            new_len + 2 - (line.len() as i32)
        })
        .sum()
}
