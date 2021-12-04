use std::io;
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
    point: Point,
    keys: BTreeSet<char>
}

impl VisitNode {
    fn new(point: Point, keys: BTreeSet<char>) -> Self {
        Self { point, keys }
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
}

struct SearchNode {
    visit_node: VisitNode,
    dist: usize,
}

impl SearchNode {
    fn new(point: Point, keys: BTreeSet<char>, dist: usize) -> Self {
        Self { visit_node: VisitNode::new(point, keys), dist }
    }

    fn can_access(&self, tile: TileType) -> bool {
        self.visit_node.can_access(tile)
    }

    fn add_key(&mut self, key: char) {
        self.visit_node.add_key(key)
    }

    fn point(&self) -> &Point {
        &self.visit_node.point
    }

    fn adjacent(&self) -> Vec<SearchNode> {
        self.visit_node.point.adjacent().into_iter().map(|point| SearchNode::new(point, self.visit_node.keys.clone(), self.dist + 1)).collect()
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

struct GameMap {
    map: HashMap<Point, TileType>,
    player_pos: Point,
    key_count: usize,
}

impl GameMap {
    fn new() -> Self {
        Self { map: HashMap::new(), player_pos: Point::new(0, 0), key_count: 0 }
    }

    fn get_tile(&self, p: &Point) -> TileType {
        self.map[p]
    }

    fn add_node(&mut self, x: usize, y: usize, c: char) {
        let point = Point::new(x, y);
        let tile = TileType::from_char(c);
        if c == '@' {
            self.player_pos = point;
        }
        match tile {
            TileType::Key(_) => self.key_count += 1,
            _ => (),
        }
        self.map.insert(point, tile);
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

    let mut to_visit: Vec<SearchNode> = vec![SearchNode::new(map.player_pos, BTreeSet::new(), 0)];
    let mut visited: HashSet<VisitNode> = HashSet::new();
    let mut min_steps = std::usize::MAX;
    let mut dist_trav = 0;

    while to_visit.len() > 0 {
        let node = to_visit.pop().unwrap();
        if visited.contains(&node.visit_node) {
            continue
        }

        visited.insert(node.visit_node.clone());

        if node.visit_node.keys.len() == map.key_count {
            min_steps = node.dist;
            break;
        }

        if dist_trav < node.dist {
            dist_trav = node.dist;
            println!("Travelled {} tiles - to visit {}", dist_trav, to_visit.len());
        }

        node.adjacent()
            .into_iter()
            .filter(|next_node| next_node.can_access(map.get_tile(next_node.point())))
            .for_each(|mut next_node| {
                match map.get_tile(next_node.point()) {
                    TileType::Key(c) => next_node.add_key(c),
                    _ => (),
                }
                to_visit.insert(0, next_node);
            });
    };

    min_steps
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
