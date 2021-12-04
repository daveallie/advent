use std::{cmp, io};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn adjacent(&self) -> Vec<Self> {
        let mut output = Vec::with_capacity(4);
        output.push(Self { x: self.x + 1, y: self.y });
        output.push(Self { x: self.x, y: self.y +1 });

        if self.x > 0 {
            output.push(Self { x: self.x - 1, y: self.y });
        }
        if self.y > 0 {
            output.push(Self { x: self.x, y: self.y - 1 });
        }

        output
    }
}

#[derive(Debug)]
enum TileType {
    Empty,
    Wall,
    Key(char),
    Lock(char)
}

impl TileType {
    fn from_char(c: char) -> Self {
        match c {
            '.' | '@' => Self::Empty,
            '#' => Self::Wall,
            'A'..='Z' => Self::Lock(((c as u8) - b'A' + b'a') as char),
            'a'..='z' => Self::Key(c),
            _ => panic!("Unknown tile type"),
        }
    }
}

#[derive(Debug)]
struct Node {
    tile: TileType,
    point: Point,
}

impl Node {
    fn new(point: Point, c: char) -> Self {
        Self { point, tile: TileType::from_char(c) }
    }
}

#[derive(Clone, Debug)]
struct SysState {
    held_keys: HashSet<char>,
//    held_keys: Vec<char>,
    pos: char,
    steps_taken: usize,
}

#[derive(Debug, Clone)]
struct Target {
    dist: usize,
    keys_needed: HashSet<char>,
}

#[derive(Debug, Clone)]
struct TargetKey {
    key: char,
    target: Target,
}

fn main() {
    println!("{}", solve(false));
}

fn solve(is_v2: bool) -> usize {
    let mut map: HashMap<Point, Node> = HashMap::new();
    let mut key_locs: HashSet<Point> = HashSet::new();
    let mut y = 0;
    let mut player_pos = Point::new(0, 0);

    loop {
        let line = read_line().unwrap();
        if line == "" {
            break;
        }

        line.chars().enumerate().for_each(|(x, c)| {
            let point = Point::new(x, y);
            let node = Node::new(point, c);
            if c == '@' {
                player_pos = point;
            }
            match node.tile {
                TileType::Key(_) => { key_locs.insert(point); },
                _ => (),
            }

            map.insert(point, node);
        });

        y += 1;
    }

    let key_count = key_locs.len();
    let key_to_key_map = build_key_to_key_map(&map, &key_locs, &player_pos);
    output_key_to_key_map(&key_to_key_map);
    let mut solvable_states: Vec<SysState> = vec![SysState { held_keys: HashSet::new(), pos: '@', steps_taken: 0 }];
//    let mut solvable_states: Vec<SysState> = vec![SysState { held_keys: vec![], pos: '@', steps_taken: 0 }];
    let mut min_steps = std::usize::MAX;
    let mut iter_c = 0;

    while solvable_states.len() > 0 {
        let to_solve_state = solvable_states.pop().unwrap();
        if to_solve_state.held_keys.len() == key_count {
//            if to_solve_state.steps_taken < min_steps {
//                println!("Found solve taking {} steps - order: {}", to_solve_state.steps_taken, to_solve_state.held_keys.iter().collect::<String>());
//            }
//            solved_states.push(to_solve_state);
            min_steps = cmp::min(min_steps, to_solve_state.steps_taken);

            iter_c += 1;
            if iter_c % 10000 == 0 {
                println!("{} solns found -- best {} - {} states in queue", iter_c, min_steps, solvable_states.len());
            }
            continue;
        }

        key_to_key_map[&to_solve_state.pos].iter()
            .filter(|target_key| !to_solve_state.held_keys.contains(&target_key.key))
            .filter(|target_key| target_key.target.keys_needed.difference(&to_solve_state.held_keys).count() == 0)
//            .filter(|target_key| target_key.target.keys_needed.difference(&HashSet::from_iter(to_solve_state.held_keys.iter().cloned())).count() == 0)
            .for_each(|target_key| {
                let mut new_solveable_state = to_solve_state.clone();
//                new_solveable_state.held_keys.push(target_key.key);
                new_solveable_state.held_keys.insert(target_key.key);
                new_solveable_state.steps_taken += target_key.target.dist;
                new_solveable_state.pos = target_key.key;
//                println!("{:?}", new_solveable_state);
                solvable_states.push(new_solveable_state);
            });


//        for (key, dist, new_player_pos) in dist_to_keys(&map, &to_solve_state) {
//            let mut new_solveable_state = to_solve_state.clone();
////            new_solveable_state.held_keys.insert(key);
//            new_solveable_state.held_keys.push(key);
//            new_solveable_state.steps_taken += dist;
//            new_solveable_state.player_pos = new_player_pos;
//            solvable_states.push(new_solveable_state);
//        }
    }


//    let mut solvable_states: Vec<SysState> = vec![SysState { held_keys: HashSet::new(), player_pos, steps_taken: 0 }];
//    let mut solvable_states: Vec<SysState> = vec![SysState { held_keys: vec![], player_pos, steps_taken: 0 }];
//    let mut min_steps = std::usize::MAX;
////    let mut solved_states: Vec<SysState> = vec![];
//
//    while solvable_states.len() > 0 {
//        let to_solve_state = solvable_states.pop().unwrap();
//        if to_solve_state.held_keys.len() == key_count {
//            if to_solve_state.steps_taken < min_steps {
//                println!("Found new best solve taking {} steps - order: {}", to_solve_state.steps_taken, to_solve_state.held_keys.iter().collect::<String>());
//            }
////            solved_states.push(to_solve_state);
//            min_steps = cmp::min(min_steps, to_solve_state.steps_taken);
//            continue;
//        }
//
//        for (key, dist, new_player_pos) in dist_to_keys(&map, &to_solve_state) {
//            let mut new_solveable_state = to_solve_state.clone();
////            new_solveable_state.held_keys.insert(key);
//            new_solveable_state.held_keys.push(key);
//            new_solveable_state.steps_taken += dist;
//            new_solveable_state.player_pos = new_player_pos;
//            solvable_states.push(new_solveable_state);
//        }
//    }

//    for s_state in solved_states {
//    }

//    println!("{:?}", map);
    min_steps
//    0
}

fn output_key_to_key_map(key_to_key_map: &HashMap<char, Vec<TargetKey>>) {
    let mut keys: Vec<char> = key_to_key_map.keys().cloned().collect();
    keys.sort();
    for key in keys {
        println!("{}", key);
        let mut vals = key_to_key_map[&key].clone();
//        vals.sort_by(|a, b| a.key.partial_cmp(&b.key).unwrap());
        for val in vals {
            println!("{:?}", val);
        }
        println!();
    }
}

fn build_key_to_key_map(map: &HashMap<Point, Node>, key_locs: &HashSet<Point>, player_pos: &Point) -> HashMap<char, Vec<TargetKey>> {
    let mut output = HashMap::new();
    let mut starts = key_locs.clone();
    starts.insert(player_pos.clone());

    for start in starts {
        let mut to_look: Vec<Point> = vec![start];
        let mut visited: HashSet<Point> = HashSet::new();
        let mut targets: HashMap<Point, Target> = HashMap::new();
        targets.insert(start, Target { dist: 0, keys_needed: HashSet::new() });

        while to_look.len() > 0 {
            let next_point = to_look.pop().unwrap();
            if visited.contains(&next_point) {
                continue
            }
            visited.insert(next_point);

            match map[&next_point].tile {
                TileType::Wall => continue,
                _ => ()
            }

            for p in next_point.adjacent().iter() {
                if targets.contains_key(p) {
                    continue;
                }

                let mut new_t = targets[&next_point].clone();
                new_t.dist += 1;

                match map[p].tile {
                    TileType::Lock(c) => {
                        new_t.keys_needed.insert(c);
                    },
                    _ => (),
                }

                let p = p.clone();
                to_look.insert(0, p);
                targets.insert(p, new_t);
            }
        }

        let output_key = match map[&start].tile {
            TileType::Key(c) => c,
            TileType::Empty => '@',
            _ => unreachable!(),
        };

        let mut value: Vec<TargetKey> = key_locs.iter()
            .map(|finish| {
                match map[&finish].tile {
                    TileType::Key(c) => (c, finish),
                    TileType::Empty => ('@', finish),
                    _ => unreachable!(),
                }
            })
            .filter(|(key, _)| key != &output_key)
            .map(|(key, finish)| TargetKey { key, target: targets[finish].clone() })
            .collect();

        value.sort_by(|a, b| b.target.dist.cmp(&a.target.dist));

        output.insert(output_key, value);
    }

    output
}

//fn dist_to_keys(map: &HashMap<Point, Node>, sys_state: &SysState) -> Vec<(char, usize, Point)> {
//    let mut to_look: Vec<Point> = vec![sys_state.player_pos];
//    let mut visited: HashSet<Point> = HashSet::new();
//    let mut dist: HashMap<Point, usize> = HashMap::new();
//    dist.insert(sys_state.player_pos, 0);
//
//    while to_look.len() > 0 {
//        let next_point = to_look.pop().unwrap();
//        if visited.contains(&next_point) {
//            continue
//        }
//
//        visited.insert(next_point);
//
//        match map[&next_point].tile {
//            TileType::Wall => {
//                continue;
//            },
//            TileType::Lock(c) | TileType::Key(c) if !sys_state.held_keys.contains(&c) => {
//                continue;
//            },
//            _ => (),
//        }
//
//        for p in next_point.adjacent().iter() {
//            let p = p.clone();
//            to_look.insert(0, p);
//            dist.insert(p, dist[&next_point] + 1);
//        }
//    };
//
//    map.iter()
//        .filter_map(|(point, node)| {
//            match node.tile {
//                TileType::Key(c) if !sys_state.held_keys.contains(&c) && dist.contains_key(point) => Some((c, point)),
//                _ => None
//            }
//        })
//        .map(|(c, point)| (c, dist[point], point.clone()))
//        .collect()
//}

fn read_line() -> Result<String, io::Error> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.pop();
            Ok(input)
        }
        Err(error) => Err(error),
    }
}
