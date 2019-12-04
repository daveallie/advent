use std::io;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    steps_to_point: i32,
}

impl Point {
    fn manhattan_dist(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

#[derive(Debug)]
struct VLine {
    top: i32,
    bottom: i32,
    hor_pos: i32,
    start_point: Point,
}

#[derive(Debug)]
struct HLine {
    left: i32,
    right: i32,
    ver_pos: i32,
    start_point: Point,
}

fn main() {
    println!("{}", solve(true));
}

fn solve(is_v2: bool) -> i32 {
    let (c1_v_lines, c1_h_lines) = process_line(&read_line().unwrap());
    let (c2_v_lines, c2_h_lines) = process_line(&read_line().unwrap());
    let mut intersections = find_intersections(&c1_v_lines[..], &c2_h_lines[..]);
    let mut intersections2 = find_intersections(&c2_v_lines[..], &c1_h_lines[..]);
    intersections.append(&mut intersections2);

    let filtered_points = intersections.into_iter().filter(|i| !i.is_origin());

    if is_v2 {
        filtered_points.map(|i| i.steps_to_point).min().unwrap()
    } else {
        filtered_points.map(|i| i.manhattan_dist()).min().unwrap()
    }
}

fn process_line(line: &str) -> (Vec<VLine>, Vec<HLine>) {
    let mut curr_point = Point {
        x: 0,
        y: 0,
        steps_to_point: 0,
    };
    let mut v_lines = vec![];
    let mut h_lines = vec![];

    line.split(",").for_each(|inst| {
        let dir: &str = &inst[0..1];
        let amt: i32 = (&inst[1..]).parse().unwrap();

        match dir {
            "U" => {
                v_lines.push(VLine {
                    top: curr_point.y + amt,
                    bottom: curr_point.y,
                    hor_pos: curr_point.x,
                    start_point: curr_point,
                });
                curr_point = Point {
                    x: curr_point.x,
                    y: curr_point.y + amt,
                    steps_to_point: curr_point.steps_to_point + amt,
                };
            }
            "D" => {
                v_lines.push(VLine {
                    top: curr_point.y,
                    bottom: curr_point.y - amt,
                    hor_pos: curr_point.x,
                    start_point: curr_point,
                });
                curr_point = Point {
                    x: curr_point.x,
                    y: curr_point.y - amt,
                    steps_to_point: curr_point.steps_to_point + amt,
                };
            }
            "R" => {
                h_lines.push(HLine {
                    left: curr_point.x,
                    right: curr_point.x + amt,
                    ver_pos: curr_point.y,
                    start_point: curr_point,
                });
                curr_point = Point {
                    x: curr_point.x + amt,
                    y: curr_point.y,
                    steps_to_point: curr_point.steps_to_point + amt,
                };
            }
            "L" => {
                h_lines.push(HLine {
                    left: curr_point.x - amt,
                    right: curr_point.x,
                    ver_pos: curr_point.y,
                    start_point: curr_point,
                });
                curr_point = Point {
                    x: curr_point.x - amt,
                    y: curr_point.y,
                    steps_to_point: curr_point.steps_to_point + amt,
                };
            }
            _ => unreachable!(),
        }
    });

    v_lines.sort_by(|a, b| b.top.cmp(&a.top));
    h_lines.sort_by(|a, b| b.ver_pos.cmp(&a.ver_pos));

    (v_lines, h_lines)
}

fn find_intersections(v_lines: &[VLine], h_lines: &[HLine]) -> Vec<Point> {
    let mut intersections = vec![];

    if v_lines.len() == 0 || h_lines.len() == 0 {
        return intersections;
    }

    let mut v_lines_index: usize = 0;
    let mut h_lines_index: usize = 0;

    while v_lines_index < v_lines.len() {
        let v_line = &v_lines[v_lines_index.clone()];

        while h_lines_index < h_lines.len() && h_lines[h_lines_index.clone()].ver_pos > v_line.top {
            h_lines_index += 1;
        }

        if h_lines_index >= h_lines.len() {
            v_lines_index += 1;
            continue;
        }

        let mut curr_h_index = h_lines_index.clone();

        while curr_h_index < h_lines.len() && h_lines[curr_h_index].ver_pos >= v_line.bottom {
            let h_line = &h_lines[curr_h_index.clone()];

            if h_line.left <= v_line.hor_pos && v_line.hor_pos <= h_line.right {
                let v_line_dist = v_line.start_point.steps_to_point
                    + (v_line.start_point.y - h_line.ver_pos).abs();
                let h_line_dist = h_line.start_point.steps_to_point
                    + (h_line.start_point.x - v_line.hor_pos).abs();

                intersections.push(Point {
                    x: v_line.hor_pos,
                    y: h_line.ver_pos,
                    steps_to_point: v_line_dist + h_line_dist,
                });
            }

            curr_h_index += 1;
        }

        v_lines_index += 1;
    }

    intersections
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
