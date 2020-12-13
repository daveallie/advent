struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn translate(&mut self, heading: Heading, amount: i32) {
        match heading {
            Heading::North => self.y -= amount,
            Heading::East => self.x += amount,
            Heading::South => self.y += amount,
            Heading::West => self.x -= amount,
        }
    }
}

#[derive(Copy, Clone)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    fn to_num(&self) -> i32 {
        match self {
            Self::North => 0,
            Self::East => 90,
            Self::South => 180,
            Self::West => 270,
        }
    }

    fn from_num(num: i32) -> Self {
        let num = if num < 0 {
            num + ((-1 * num / 360 + 1) * 360)
        } else {
            num
        };

        match num % 360 {
            0 => Self::North,
            90 => Self::East,
            180 => Self::South,
            270 => Self::West,
            _ => panic!(),
        }
    }

    fn right(&self, amount: i32) -> Self {
        Self::from_num(self.to_num() + amount)
    }

    fn left(&self, amount: i32) -> Self {
        Self::from_num(self.to_num() - amount)
    }
}

struct Waypoint {
    position: Point,
}

impl Waypoint {
    fn rotate_cw(&mut self, amount: i32) {
        let amount = if amount < 0 {
            amount + ((-1 * amount / 360 + 1) * 360)
        } else {
            amount
        };

        match amount % 360 {
            0 => (),
            90 => {
                let old_x = self.position.x;
                self.position.x = self.position.y * -1;
                self.position.y = old_x;
            },
            180 => {
                self.position.x = self.position.x * -1;
                self.position.y = self.position.y * -1;
            },
            270 => {
                let old_x = self.position.x;
                self.position.x = self.position.y;
                self.position.y = old_x * -1;
            },
            _ => panic!(),
        }
    }

    fn rotate_ccw(&mut self, amount: i32) {
        self.rotate_cw(360 - amount);
    }
}

struct Ferry {
    position: Point,
    heading: Heading,
    waypoint: Waypoint,
}

impl Ferry {
    fn new() -> Self {
        Self {
            position: Point { x: 0, y: 0 },
            heading: Heading::East,
            waypoint: Waypoint { position: Point { x: 10, y: -1 } }
        }
    }

    fn forward(&mut self, amount: i32) {
        self.position.translate(self.heading, amount);
    }

    fn towards_waypoint(&mut self, amount: i32) {
        self.position.x += self.waypoint.position.x * amount;
        self.position.y += self.waypoint.position.y * amount;
    }
}

#[aoc(day12, part1)]
fn solve_part1(input: &str) -> i32 {
    let mut ferry = Ferry::new();

    input.lines()
        .for_each(|line| {
            let mut chars = line.chars();
            let action = chars.next().unwrap();
            let value = chars.collect::<String>().parse::<i32>().unwrap();

            match action {
                'N' => ferry.position.translate(Heading::North, value),
                'E' => ferry.position.translate(Heading::East, value),
                'S' => ferry.position.translate(Heading::South, value),
                'W' => ferry.position.translate(Heading::West, value),
                'L' => ferry.heading = ferry.heading.left(value),
                'R' => ferry.heading = ferry.heading.right(value),
                'F' => ferry.forward(value),
                _ => panic!(),
            }
        });

    ferry.position.x.abs() + ferry.position.y.abs()
}

#[aoc(day12, part2)]
fn solve_part2(input: &str) -> i32 {
    let mut ferry = Ferry::new();

    input.lines()
        .for_each(|line| {
            let mut chars = line.chars();
            let action = chars.next().unwrap();
            let value = chars.collect::<String>().parse::<i32>().unwrap();

            match action {
                'N' => ferry.waypoint.position.translate(Heading::North, value),
                'E' => ferry.waypoint.position.translate(Heading::East, value),
                'S' => ferry.waypoint.position.translate(Heading::South, value),
                'W' => ferry.waypoint.position.translate(Heading::West, value),
                'L' => ferry.waypoint.rotate_ccw(value),
                'R' => ferry.waypoint.rotate_cw(value),
                'F' => ferry.towards_waypoint(value),
                _ => panic!(),
            }
        });

    ferry.position.x.abs() + ferry.position.y.abs()
}
