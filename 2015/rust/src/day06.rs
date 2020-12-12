use std::str::FromStr;

struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s.split(",").map(|v| v.parse::<usize>().unwrap()).collect::<Vec<usize>>();

        Ok(Self {
            x: coords[0],
            y: coords[1],
        })
    }
}

enum Action {
    On,
    Off,
    Toggle,
}

struct Task {
    point1: Point,
    point2: Point,
    action: Action,
}

impl FromStr for Task {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let action = match words.next().unwrap() {
            "toggle" => Action::Toggle,
            "turn" => match words.next().unwrap() {
                "on" => Action::On,
                "off" => Action::Off,
                _ => panic!(),
            },
            _ => panic!(),
        };

        let point1 = Point::from_str(words.next().unwrap()).unwrap();
        words.next().unwrap();
        let point2 = Point::from_str(words.next().unwrap()).unwrap();

        Ok(Self {
            point1,
            point2,
            action,
        })
    }
}

struct Grid {
    cells: Vec<Vec<usize>>,
}

impl Grid {
    fn new() -> Self {
        let cells = (0..1000).map(|_| (0..1000).map(|_| 0).collect()).collect();
        Self { cells }
    }

    fn perform_task(&mut self, task: &Task, part2: bool) {
        for x in task.point1.x..=task.point2.x {
            for y in task.point1.y..=task.point2.y {
                let cell = self.cells.get_mut(x).map(|row| row.get_mut(y)).flatten().unwrap();

                if part2 {
                    match task.action {
                        Action::Toggle => *cell += 2,
                        Action::On => *cell += 1,
                        Action::Off if *cell > 0 => *cell -= 1,
                        Action::Off if *cell <= 0 => *cell = 0,
                        _ => {}
                    }
                } else {
                    match task.action {
                        Action::Toggle => *cell = if *cell == 1 { 0 } else { 1 },
                        Action::On => *cell = 1,
                        Action::Off => *cell = 0,
                    }
                }
            }
        }
    }

    fn count_on(&self) -> usize {
        self.cells.iter()
            .flat_map(|row| row.iter())
            .sum()
    }
}

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<Task> {
    input.lines().map(|line| Task::from_str(line).unwrap()).collect()
}

#[aoc(day6, part1)]
fn solve_part1(tasks: &[Task]) -> usize {
    let mut grid = Grid::new();
    for task in tasks {
        grid.perform_task(task, false)
    }

    grid.count_on()
}

#[aoc(day6, part2)]
fn solve_part2(tasks: &[Task]) -> usize {
    let mut grid = Grid::new();
    for task in tasks {
        grid.perform_task(task, true)
    }

    grid.count_on()
}
