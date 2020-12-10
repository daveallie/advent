use std::collections::HashMap;

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<usize> {
    let mut v = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>();

    v.push(0);
    v.sort();
    v.push(v[v.len() - 1] + 3);
    v
}

#[aoc(day10, part1)]
fn solve_part1(input: &[usize]) -> usize {
    let mut jumps = HashMap::new();

    for w in input.windows(2) {
        *jumps.entry(w[1] - w[0]).or_insert(0) += 1;
    }

    jumps[&1] * jumps[&3]
}

#[aoc(day10, part2)]
fn solve_part2(input: &[usize]) -> usize {
    let mut index_to_arrangements = HashMap::new();
    index_to_arrangements.insert(input.len() - 1, 1);
    calc_arrangements(0, input, &mut index_to_arrangements);
    index_to_arrangements[&0]
}

fn calc_arrangements(index: usize, input: &[usize], index_to_arrangements: &mut HashMap<usize, usize>) {
    if index_to_arrangements.contains_key(&index) {
        return
    }

    let curr = input[index];
    let mut total = 0;
    for i in index+1..input.len() {
        if curr + 3 < input[i]  {
            break
        }

        calc_arrangements(i, input, index_to_arrangements);
        total += index_to_arrangements[&i];
    }

    index_to_arrangements.insert(index, total);
}
