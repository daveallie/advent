use std::collections::HashSet;

#[aoc(day3, part1)]
fn solve_part1(input: &str) -> usize {
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut pos = (0, 0);
    seen.insert(pos);

    input.chars().for_each(|c| {
        pos = get_next_pos(pos, c);
        seen.insert(pos);
    });

    seen.len()
}

#[aoc(day3, part2)]
fn solve_part2(input: &str) -> usize {
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut pos = [(0, 0), (0, 0)];
    seen.insert(pos[0]);

    input.chars().enumerate().for_each(|(i, c)| {
        let index = i % 2;
        pos[index] = get_next_pos(pos[index], c);
        seen.insert(pos[index]);
    });

    seen.len()
}

fn get_next_pos(pos: (i32, i32), c: char) -> (i32, i32) {
    match c {
        '^' => (pos.0 - 1, pos.1),
        'v' => (pos.0 + 1, pos.1),
        '<' => (pos.0, pos.1 - 1),
        '>' => (pos.0, pos.1 + 1),
        _ => panic!("unknown direction"),
    }
}
