use std::{cmp, io};
use std::collections::{BTreeSet, HashMap, HashSet};
//use std::iter::FromIterator;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
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

#[derive(Eq, PartialEq, Clone, Hash)]
struct VisitNode {
    pos: char,
    keys: BTreeSet<char>
}

impl VisitNode {
    fn new(pos: char, keys: BTreeSet<char>) -> Self {
        Self { pos, keys }
    }

    fn can_access(&self, tile: TileType) -> bool {
        match tile {
            TileType::Empty | TileType::Key(_) => true,
            TileType::Wall => false,
            TileType::Lock(c) => self.keys.contains(&c),
        }
    }

    fn add_key(&mut self, key: char) {
        self.keys.insert(key);
    }

    fn has_needed_keys(&self, needed_keys: &BTreeSet<char>) -> bool {
        needed_keys.difference(&self.keys).count() == 0
    }
}

#[derive(Eq, PartialEq, Clone, Hash)]
struct KeyTargetVisitNode {
    point: Point,
    keys_needed: BTreeSet<char>,
}

impl KeyTargetVisitNode {
    fn new(point: Point, keys_needed: BTreeSet<char>) -> Self {
        Self { point, keys_needed }
    }

    fn add_needed_key(&mut self, key: char) {
        self.keys_needed.insert(key);
    }
}

#[derive(Clone)]
struct SearchNode {
    visit_node: VisitNode,
    dist: usize,
}

impl SearchNode {
    fn new(pos: char, keys: BTreeSet<char>, dist: usize) -> Self {
        Self { visit_node: VisitNode::new(pos, keys), dist }
    }

    fn can_access(&self, tile: TileType) -> bool {
        self.visit_node.can_access(tile)
    }

    fn add_key(&mut self, key: char) {
        self.visit_node.add_key(key)
    }

    fn pos(&self) -> char {
        self.visit_node.pos
    }

    fn has_needed_keys(&self, needed_keys: &BTreeSet<char>) -> bool {
        self.visit_node.has_needed_keys(needed_keys)
    }

//    fn adjacent(&self) -> Vec<Self> {
//        self.visit_node.point.adjacent().into_iter().map(|point| Self::new(point, self.visit_node.keys.clone(), self.dist + 1)).collect()
//    }
}

struct KeyTargetSearchNode {
    visit_node: KeyTargetVisitNode,
    dist: usize,
}

impl KeyTargetSearchNode {
    fn new(point: Point, keys_needed: BTreeSet<char>, dist: usize) -> Self {
        Self { visit_node: KeyTargetVisitNode::new(point, keys_needed), dist }
    }

    fn add_needed_key(&mut self, key: char) {
        self.visit_node.add_needed_key(key)
    }

    fn point(&self) -> &Point {
        &self.visit_node.point
    }

    fn adjacent(&self) -> Vec<Self> {
        self.visit_node.point.adjacent().into_iter().map(|point| Self::new(point, self.visit_node.keys_needed.clone(), self.dist + 1)).collect()
    }
}

#[derive(Copy, Clone)]
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

#[derive(Clone, Debug)]
struct TargetKey {
    key: char,
    dist: usize,
    keys_needed: BTreeSet<char>
}

impl TargetKey {
    fn new(key: char, dist: usize, keys_needed: BTreeSet<char>) -> Self {
        Self { key, dist, keys_needed }
    }
}

struct GameMap {
    map: HashMap<Point, TileType>,
    key_locs: HashMap<char, Point>,
    key_to_key_map: HashMap<char, Vec<TargetKey>>,
    player_pos: Point,
}

impl GameMap {
    fn new() -> Self {
        Self { map: HashMap::new(), key_to_key_map: HashMap::new(), player_pos: Point::new(0, 0), key_locs: HashMap::new() }
    }

    fn get_tile(&self, p: &Point) -> TileType {
        self.map[p]
    }

    fn key_count(&self) -> usize {
        self.key_locs.len()
    }

    fn add_node(&mut self, x: usize, y: usize, c: char) {
        let point = Point::new(x, y);
        let tile = TileType::from_char(c);
        if c == '@' {
            self.player_pos = point;
        }
        match tile {
            TileType::Key(c) => { self.key_locs.insert(c, point); },
            _ => (),
        }
        self.map.insert(point, tile);
    }

    fn add_key_to_key_entry(&mut self, from: char, target_key: TargetKey) {
        if !self.key_to_key_map.contains_key(&from) {
            self.key_to_key_map.insert(from, vec![]);
        }
        self.key_to_key_map.get_mut(&from).unwrap().push(target_key);
    }
}

fn main() {
    println!("{}", solve(false));
}

fn solve(is_v2: bool) -> usize {
    let mut map: GameMap = GameMap::new();

    {
        let mut y = 0;
        loop {
            let line = read_line().unwrap();
            if line == "" {
                break;
            }

            line.chars()
                .enumerate()
                .for_each(|(x, c)| map.add_node(x, y, c));
            y += 1;
        }
    }

    build_key_to_key_map(&mut map);
    output_key_to_key_map(&map.key_to_key_map);

    let mut to_visit: Vec<SearchNode> = vec![SearchNode::new('@', BTreeSet::new(), 0)];
    let mut visited: HashSet<VisitNode> = HashSet::new();
    let mut min_steps = std::usize::MAX;
//    let mut dist_trav = 0;

    while to_visit.len() > 0 {
        let node = to_visit.pop().unwrap();
        if visited.contains(&node.visit_node) {
            continue
        }

        visited.insert(node.visit_node.clone());

        if node.visit_node.keys.len() == map.key_count() {
            min_steps = cmp::min(min_steps, node.dist);
            println!("Found {} - Min {}", node.dist, min_steps);
            continue;
        }

//        if dist_trav < node.dist {
//            dist_trav = node.dist;
//            println!("Travelled {} tiles - to visit {}", dist_trav, to_visit.len());
//        }

        map.key_to_key_map[&node.pos()].iter()
            .filter(|target_key| !node.visit_node.keys.contains(&target_key.key))
            .filter(|target_key| node.has_needed_keys(&target_key.keys_needed))
            .for_each(|target_key| {
                let mut next_node = node.clone();
                next_node.visit_node.pos = target_key.key;
                next_node.add_key(target_key.key);
                next_node.dist += target_key.dist;
//                to_visit.insert(0, next_node);
                to_visit.push(next_node);
            });
    };

    min_steps
}

fn output_key_to_key_map(key_to_key_map: &HashMap<char, Vec<TargetKey>>) {
    let mut keys: Vec<char> = key_to_key_map.keys().cloned().collect();
    keys.sort();
    for key in keys {
        println!("{}", key);
        let mut vals = key_to_key_map[&key].clone();
        vals.sort_by(|a, b| a.key.partial_cmp(&b.key).unwrap());
        for val in vals {
            println!("{:?}", val);
        }
        println!();
    }
}

fn build_key_to_key_map(map: &mut GameMap) {
    let mut starts: Vec<Point> = map.key_locs.values().cloned().collect();
    starts.push(map.player_pos.clone());

    for start in starts {
        let start_char = match map.get_tile(&start) {
            TileType::Key(c) => c,
            TileType::Empty => '@',
            _ => unreachable!(),
        };
        let mut to_visit: Vec<KeyTargetSearchNode> = vec![KeyTargetSearchNode::new(start, BTreeSet::new(), 0)];
        let mut visited: HashSet<KeyTargetVisitNode> = HashSet::new();
        let mut keys_found: HashSet<char> = HashSet::new();
        keys_found.insert(start_char);

        while to_visit.len() > 0 {
            let node = to_visit.pop().unwrap();
            if visited.contains(&node.visit_node) {
                continue
            }

            visited.insert(node.visit_node.clone());

            match map.get_tile(node.point()) {
                TileType::Wall => continue,
                TileType::Key(c) if !keys_found.contains(&c) => {
                    map.add_key_to_key_entry(start_char, TargetKey::new(c, node.dist, node.visit_node.keys_needed.clone()));
                    keys_found.insert(c);
                },
                _ => ()
            }

            node.adjacent()
                .into_iter()
                .for_each(|mut next_node| {
                    match map.get_tile(next_node.point()) {
                        TileType::Lock(c) => next_node.add_needed_key(c),
                        _ => (),
                    }
                    to_visit.insert(0, next_node);
                });
        }
    }
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
