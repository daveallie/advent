use std::collections::{HashMap, HashSet};

struct BagContainer {
    bag: String,
    count: usize,
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> [HashMap<String, Vec<BagContainer>>; 2] {
    let mut forward_map: HashMap<String, Vec<BagContainer>> = HashMap::new();
    let mut back_map: HashMap<String, Vec<BagContainer>> = HashMap::new();

    input
        .lines()
        .for_each(|line| {
            let mut line_parts = line.split(" bags contain ");
            let source = line_parts.next().unwrap().to_string();
            let dests = line_parts.next().unwrap();

            if dests != "no other bags." {
                dests.split(", ").for_each(|dest| {
                    let mut dest_parts = dest.split_whitespace();
                    let count = dest_parts.next().unwrap().parse().unwrap();
                    let bag = dest_parts.take(2).collect::<Vec<&str>>().join(" ");

                    forward_map.entry(source.clone()).or_insert(vec![]).push(BagContainer { bag: bag.clone(), count });
                    back_map.entry(bag).or_insert(vec![]).push(BagContainer { bag: source.clone(), count });
                });
            }
        });

    [forward_map, back_map]
}

#[aoc(day7, part1)]
fn solve_part1(maps: &[HashMap<String, Vec<BagContainer>>]) -> usize {
    let map = maps.get(1).unwrap(); // back_map
    let mut stack: Vec<String> = vec!["shiny gold".to_string()];
    let mut seen: HashSet<String> = HashSet::new();

    while !stack.is_empty() {
        let next = stack.pop().unwrap();
        if seen.contains(&next) {
            continue
        }
        seen.insert(next.clone());

        for bags in map.get(&next) {
            for bag in bags {
                stack.push(bag.bag.clone())
            }
        }
    }

    seen.len() - 1
}

#[aoc(day7, part2)]
fn solve_part2(maps: &[HashMap<String, Vec<BagContainer>>]) -> usize {
    let map = maps.get(0).unwrap(); // forward_map
    let mut stack: Vec<BagContainer> = vec![BagContainer { bag: "shiny gold".to_string(), count: 1 }];
    let mut total = 0;

    while !stack.is_empty() {
        let next = stack.pop().unwrap();

        total += next.count;

        for bags in map.get(&next.bag) {
            for bag in bags {
                stack.push(BagContainer { bag: bag.bag.clone(), count: next.count * bag.count })
            }
        }
    }

    total - 1
}
