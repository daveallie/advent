use std::{cmp, io};
use std::collections::{HashMap, HashSet};

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn adjacent(&self) -> [Self; 4] {
        [
            Self { x: cmp::max(self.x - 1, 0), y: self.y },
            Self { x: self.x + 1, y: self.y },
            Self { x: self.x, y: cmp::max(self.y - 1, 0) },
            Self { x: self.x, y: self.y + 1 },
        ]
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

#[derive(Clone)]
struct SysState {
    held_keys: Vec<char>,
    player_pos: Point,
    steps_taken: usize,
}

fn main() {
    println!("{}", solve(false));
}

fn solve(is_v2: bool) -> usize {
    let mut map: HashMap<Point, Node> = HashMap::new();
    let mut y = 0;
    let mut player_pos = Point::new(0, 0);
    let mut key_count = 0;

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
                TileType::Key(_) => key_count += 1,
                _ => (),
            }

            map.insert(point, node);
        });

        y += 1;
    }

//    let mut solvable_states: Vec<SysState> = vec![SysState { held_keys: HashSet::new(), player_pos, steps_taken: 0 }];
    let mut solvable_states: Vec<SysState> = vec![SysState { held_keys: vec![], player_pos, steps_taken: 0 }];
    let mut min_steps = std::usize::MAX;
//    let mut solved_states: Vec<SysState> = vec![];

    while solvable_states.len() > 0 {
        let to_solve_state = solvable_states.pop().unwrap();
        if to_solve_state.held_keys.len() == key_count {
            if to_solve_state.steps_taken < min_steps {
                println!("Found new best solve taking {} steps - order: {}", to_solve_state.steps_taken, to_solve_state.held_keys.iter().collect::<String>());
            }
//            solved_states.push(to_solve_state);
            min_steps = cmp::min(min_steps, to_solve_state.steps_taken);
            continue;
        }

        for (key, dist, new_player_pos) in dist_to_keys(&map, &to_solve_state) {
            let mut new_solveable_state = to_solve_state.clone();
//            new_solveable_state.held_keys.insert(key);
            new_solveable_state.held_keys.push(key);
            new_solveable_state.steps_taken += dist;
            new_solveable_state.player_pos = new_player_pos;
            solvable_states.push(new_solveable_state);
        }
    }

//    for s_state in solved_states {
//    }

//    println!("{:?}", map);
    min_steps
}

fn dist_to_keys(map: &HashMap<Point, Node>, sys_state: &SysState) -> Vec<(char, usize, Point)> {
    let mut to_look: Vec<Point> = vec![sys_state.player_pos];
    let mut visited: HashSet<Point> = HashSet::new();
    let mut dist: HashMap<Point, usize> = HashMap::new();
    dist.insert(sys_state.player_pos, 0);

    while to_look.len() > 0 {
        let next_point = to_look.pop().unwrap();
        if visited.contains(&next_point) {
            continue
        }

        visited.insert(next_point);

        match map[&next_point].tile {
            TileType::Wall => {
                continue;
            },
            TileType::Lock(c) | TileType::Key(c) if !sys_state.held_keys.contains(&c) => {
                continue;
            },
            _ => (),
        }

        for p in next_point.adjacent().iter() {
            let p = p.clone();
            to_look.insert(0, p);
            dist.insert(p, dist[&next_point] + 1);
        }
    };

    map.iter()
        .filter_map(|(point, node)| {
            match node.tile {
                TileType::Key(c) if !sys_state.held_keys.contains(&c) && dist.contains_key(point) => Some((c, point)),
                _ => None
            }
        })
        .map(|(c, point)| (c, dist[point], point.clone()))
        .collect()
}

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
