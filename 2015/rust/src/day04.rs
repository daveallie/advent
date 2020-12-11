#[aoc(day4, part1)]
fn solve_part1(input: &str) -> usize {
    solve(input, "00000")
}

#[aoc(day4, part2)]
fn solve_part2(input: &str) -> usize {
    solve(input, "000000")
}

fn solve(key: &str, target_prefix: &str) -> usize {
    let mut i = 0;
    loop {
        let str = format!("{}{}", key, i);
        let res = format!("{:02x}", md5::compute(str));

        if res.starts_with(target_prefix) {
            break
        }

        i += 1;
    }

    i
}
