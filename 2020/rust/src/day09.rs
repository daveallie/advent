use std::collections::HashSet;

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc(day9, part1)]
fn solve_part1(input: &[usize]) -> usize {
    let mut looking = HashSet::new();

    for i in 25..input.len() {
        let target = input[i];
        let mut found = false;
        looking.clear();

        for num in input[i - 25..i].iter() {
            if looking.contains(num) {
                found = true;
                break
            }
            looking.insert(target - num);
        }

        if !found {
            return target;
        }
    }

    0
}

#[aoc(day9, part2)]
fn solve_part2(input: &[usize]) -> usize {
    let target = 1212510616;
    let mut curr_nums = HashSet::new();

    for start in 0..input.len() - 1 {
        let mut remaining = target - input[start];
        curr_nums.clear();
        curr_nums.insert(input[start]);

        for i in start + 1..input.len() {
            if input[i] > remaining {
                break;
            }

            remaining -= input[i];
            curr_nums.insert(input[i]);

            if remaining == 0 {
                break;
            }
        }

        if remaining == 0 {
            break;
        }
    }

    let mut nums = curr_nums.iter().collect::<Vec<&usize>>();
    nums.sort();

    nums[0] + nums[nums.len() - 1]
}
