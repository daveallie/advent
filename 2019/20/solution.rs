use std::io;
use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct VisitPoint {
    point: Point,
    depth: usize,
}

impl VisitPoint {
    fn new(point: Point, depth: usize) -> Self {
        Self { point, depth }
    }
}

struct AdjPoint {
    point: Point,
    depth_change: i8,
}

impl AdjPoint {
    fn new(point: Point, depth_change: i8) -> Self {
        Self { point, depth_change }
    }
}

struct Node {
    adj: Vec<AdjPoint>,
}

impl Node {
    fn new() -> Self {
        Self { adj: vec![] }
    }

    fn add_adj(&mut self, point: AdjPoint) {
        self.adj.push(point);
    }
}

struct Map {
    map: HashMap<Point, Node>,
    visited: HashSet<VisitPoint>,
    dist_map: HashMap<VisitPoint, usize>,
    start: VisitPoint,
    finish: VisitPoint,
}

impl Map {
    fn new() -> Self {
        let origin_visit_point = VisitPoint::new(Point::new(0, 0), 0);
        Self { map: HashMap::new(), visited: HashSet::new(), dist_map: HashMap::new(), start: origin_visit_point.clone(), finish: origin_visit_point }
    }

    fn is_visited(&self, vpoint: &VisitPoint) -> bool {
        self.visited.contains(vpoint)
    }

    fn visit(&mut self, vpoint: VisitPoint) {
        self.visited.insert(vpoint);
    }

    fn set_dist_to(&mut self, vpoint: VisitPoint, dist: usize) {
        self.dist_map.insert(vpoint, dist);
    }

    fn get_dist(&self, vpoint: &VisitPoint) -> usize {
        *self.dist_map.get(vpoint).expect(&format!("No dist for {:?}", vpoint))
    }

    fn add_adj(&mut self, from: Point, to: Point, depth_change: i8) {
        if !self.map.contains_key(&from) {
            self.map.insert(from.clone(), Node::new());
        }
        self.map.get_mut(&from).unwrap().add_adj(AdjPoint::new(to.clone(), depth_change.clone()));

        if !self.map.contains_key(&to) {
            self.map.insert(to.clone(), Node::new());
        }
        self.map.get_mut(&to).unwrap().add_adj(AdjPoint::new(from, -depth_change));
    }

    fn get_adj(&self, vpoint: &VisitPoint) -> Vec<VisitPoint> {
        self.map[&vpoint.point].adj.iter()
            .filter_map(|adj_point| {
                let new_depth = vpoint.depth as i64 + i64::from(adj_point.depth_change);
                if new_depth >= 0 {
                    Some(VisitPoint::new(adj_point.point.clone(), new_depth as usize))
                } else {
                    None
                }
            }).collect()
    }
}

fn main() {
    println!("{}", solve(true));
}

fn solve(is_v2: bool) -> usize {
    let mut map = build_grid(is_v2);

    let mut to_visit = vec![map.start.clone()];
    while to_visit.len() > 0 {
        let point = to_visit.pop().unwrap();

        if map.is_visited(&point) {
            continue;
        }
        map.visit(point.clone());

        if point == map.finish {
            break
        }

        let dist = map.get_dist(&point);

        for adj_point in map.get_adj(&point) {
            map.set_dist_to(adj_point.clone(), dist + 1);
            to_visit.insert(0, adj_point);
        }
    }

    map.get_dist(&map.finish)
}

fn build_grid(is_v2: bool) -> Map {
    let mut output_map = Map::new();
    let mut raw_map: Vec<Vec<char>> = vec![];

    loop {
        let line = read_line().unwrap();
        if line == "" {
            break;
        }

        raw_map.push(line.chars().collect());
    }

    let mut gates: HashMap<String, Vec<AdjPoint>> = HashMap::new();

    for (y, row) in raw_map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            match *cell {
                '.' => {
                    if y > 0 && raw_map[y - 1][x] == '.' {
                        output_map.add_adj(Point::new(x, y), Point::new(x, y - 1), 0);
                    }

                    if x > 0 && raw_map[y][x - 1] == '.' {
                        output_map.add_adj(Point::new(x, y), Point::new(x - 1, y), 0);
                    }
                }
                'A'..='Z' => {
                    let gate_details = if y > 0 && ('A'..='Z').contains(&raw_map[y - 1][x]) {
                        let gate_name = format!("{}{}", raw_map[y - 1][x], cell);
                        let gate_cell = if y > 1 && raw_map[y - 2][x] == '.' {
                            let depth_change = if y <= raw_map.len() / 2 {
                                // in upper half of map and dot above gate
                                // going deeper
                                1
                            } else {
                                // in lower half of map and dot above gate
                                // going shallower
                                -1
                            };
                            AdjPoint::new(Point::new(x, y - 2), depth_change)
                        } else if y < raw_map.len() - 1 && raw_map[y + 1][x] == '.' {
                            let depth_change = if y <= raw_map.len() / 2 {
                                // in upper half of map and dot below gate
                                // going shallower
                                -1
                            } else {
                                // in lower half of map and dot below gate
                                // going deeper
                                1
                            };
                            AdjPoint::new(Point::new(x, y + 1), depth_change)

                        } else {
                            panic!("Can't find cell for gate {}", gate_name)
                        };

                        Some((gate_name, gate_cell))
                    } else if x > 0 && ('A'..='Z').contains(&raw_map[y][x - 1]) {
                        let gate_name = format!("{}{}", raw_map[y][x - 1], cell);
                        let gate_cell = if x > 1 && raw_map[y][x - 2] == '.' {
                            let depth_change = if x <= row.len() / 2 {
                                // in left half of map and dot left of gate
                                // going deeper
                                1
                            } else {
                                // in right half of map and dot left of gate
                                // going shallower
                                -1
                            };
                            AdjPoint::new(Point::new(x - 2, y), depth_change)
                        } else if x < row.len() - 1 && raw_map[y][x + 1] == '.' {
                            let depth_change = if x <= row.len() / 2 {
                                // in left half of map and dot right of gate
                                // going shallower
                                -1
                            } else {
                                // in right half of map and dot right of gate
                                // going deeper
                                1
                            };
                            AdjPoint::new(Point::new(x + 1, y), depth_change)
                        } else {
                            panic!("Can't find cell for gate {}", gate_name)
                        };

                        Some((gate_name, gate_cell))
                    } else {
                        None
                    };

                    if let Some((gate_name, gate_cell)) = gate_details {
                        if !gates.contains_key(&gate_name) {
                            gates.insert(gate_name.clone(), vec![]);
                        }
                        gates.get_mut(&gate_name).unwrap().push(gate_cell);
                    }
                },
                _ => ()
            }
        }
    }

    gates.into_iter()
        .for_each(|(gate_name, points)| {
            let mut point_iter = points.into_iter();

            match gate_name.as_str() {
                "AA" => output_map.start = VisitPoint::new(point_iter.next().unwrap().point, 0),
                "ZZ" => output_map.finish = VisitPoint::new(point_iter.next().unwrap().point, 0),
                _ => {
                    let first_adj_point = point_iter.next().unwrap();
                    let second_adj_point = point_iter.next().unwrap();
                    let depth_change = if is_v2 {
                        first_adj_point.depth_change.clone()
                    } else {
                        0
                    };
                    output_map.add_adj(first_adj_point.point, second_adj_point.point, depth_change)
                },
            }
        });

    output_map.set_dist_to(output_map.start.clone(), 0);

    output_map
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
