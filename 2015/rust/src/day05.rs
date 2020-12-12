use std::collections::HashSet;
use std::iter::FromIterator;

#[aoc(day5, part1)]
fn solve_part1(input: &str) -> usize {
    let vowels: HashSet<char> = HashSet::from_iter("aeiou".chars());
    let forbidden: HashSet<String> = HashSet::from_iter(vec!["ab".to_string(), "cd".to_string(), "pq".to_string(), "xy".to_string()]);

    input.lines()
        .filter(|line| {
            let chars = line.chars().collect::<Vec<char>>();
            let mut vowel_cnt = 0;
            let mut contains_doubles = false;
            let mut contains_forbidden = false;
            if vowels.contains(&chars[0]) {
                vowel_cnt += 1;
            }
            chars.windows(2).for_each(|w| {
                if vowels.contains(&w[1]) {
                    vowel_cnt += 1;
                }
                if !contains_doubles && w[0] == w[1] {
                    contains_doubles = true;
                }
                if !contains_forbidden && forbidden.contains(&w.iter().collect::<String>()) {
                    contains_forbidden = true;
                }
            });

            vowel_cnt >= 3 && contains_doubles && !contains_forbidden
        })
        .count()
}

#[aoc(day5, part2)]
fn solve_part2(input: &str) -> usize {
    input.lines()
        .filter(|line| {
            let chars = line.chars().collect::<Vec<char>>();
            let mut pairs: HashSet<String> = HashSet::new();
            let mut last_pair = chars[0..2].iter().collect::<String>();

            let contains_split_double = chars.windows(3).any(|w| w[0] == w[2]);

            let contains_repeat = chars.windows(2).skip(1).any(|w| {
                let pair = w.iter().collect::<String>();
                let res = pairs.contains(&pair);

                pairs.insert(last_pair.clone());
                last_pair = pair;

                res
            });

            contains_split_double && contains_repeat
        })
        .count()
}
