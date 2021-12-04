use std::collections::HashMap;
use std::hash::Hash;
use itertools::Itertools;

#[derive(Debug, Hash, Eq, PartialEq)]
struct State {
    from: u32,
    to: u32,
    including: Vec<u32>,
}

impl State {
    fn null_set(from: u32, to: u32) -> Self {
        State {
            from,
            to,
            including: vec![],
        }
    }

    fn clone_with_existing_head(&self, old_head: u32) -> Self {
        let mut new_including = self.including.clone();
        let index = new_including.iter().position(|x| *x == old_head).unwrap();
        new_including.remove(index);

        Self {
            from: self.from,
            to: old_head,
            including: new_including,
        }
    }

    fn clone_with_flipped_dirs(&self) -> Self {
        Self {
            from: self.to,
            to: self.from,
            including: self.including.clone(),
        }
    }
}

fn build_cost_map(input: &str) -> (HashMap<State, usize>, u32) {
    let mut cost_map = HashMap::new();

    let mut city_map: HashMap<String, u32> = HashMap::new();
    let mut city_count = 0;

    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        let c1 = parts.next().unwrap().to_string();
        parts.next();
        let c2 = parts.next().unwrap().to_string();

        let c1_id = city_map.entry(c1).or_insert_with(|| {
            city_count += 1;
            city_count
        }).clone();
        let c2_id = city_map.entry(c2).or_insert_with(|| {
            city_count += 1;
            city_count
        }).clone();

        let cost = parts.skip(1).next().unwrap().parse().unwrap();

        cost_map.insert(State::null_set(c1_id, c2_id), cost);
        cost_map.insert(State::null_set(c2_id, c1_id), cost);
    });

    (cost_map, city_count)
}

#[aoc(day9, part1)]
fn solve_part1(input: &str) -> usize {
    let (mut cost_map, set_size) = build_cost_map(input);
    generate_all_costs(&mut cost_map, set_size);

    let mut min = usize::max_value();

    for start in 1..set_size {
        for finish in (start + 1)..=set_size {
            let remaining = (1..=set_size).filter(|&v| v != start && v != finish).collect::<Vec<u32>>();
            let cost = *cost_map.get(&State { from: start, to: finish, including: remaining }).unwrap();
            if cost < min {
                min = cost;
            }
        }
    }

    min
}

#[aoc(day9, part2)]
fn solve_part2(input: &str) -> usize {
    let (mut cost_map, set_size) = build_cost_map(input);
    generate_all_costs(&mut cost_map, set_size);

    let mut max = usize::min_value();

    for start in 1..set_size {
        for finish in (start + 1)..=set_size {
            let remaining = (1..=set_size).filter(|&v| v != start && v != finish).collect::<Vec<u32>>();
            let cost = *cost_map.get(&State { from: start, to: finish, including: remaining }).unwrap();
            if max < cost {
                max = cost;
            }
        }
    }

    max
}

fn generate_all_costs(cost_map: &mut HashMap<State, usize>, set_size: u32) {
    for size in 3..=(set_size as usize) {
        for start in 1..set_size {
            for finish in start+1..=set_size {
                let remaining = (1..=set_size).filter(|&v| v != start && v != finish);
                for mut including in remaining.combinations(size - 2) {
                    including.sort();

                    let state = State {
                        from: start,
                        to: finish,
                        including,
                    };

                    let cost = calc_cost(&state, &cost_map);
                    cost_map.insert(state.clone_with_flipped_dirs(), cost);
                    cost_map.insert(state, cost);
                }
            }
        }
    }
}

fn calc_cost(state: &State, cost_map: &HashMap<State, usize>) -> usize {
    state.including.iter().map(|&last_head| {
        let dist = cost_map.get(&State::null_set(last_head, state.to)).unwrap();
        let path = cost_map.get(&state.clone_with_existing_head(last_head)).unwrap();
        dist + path
    }).min().unwrap()
}
