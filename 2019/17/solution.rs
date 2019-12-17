use std::collections::HashMap;
use std::io;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new_line(&mut self) {
        self.x = 0;
        self.y += 1;
    }

    fn next_char(&mut self) {
        self.x += 1;
    }

    fn build_adj(&self) -> [Point; 4] {
        [
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }
}

#[derive(PartialEq, Debug)]
enum TileType {
    Empty,
    Scaffold,
    Robot,
}

impl TileType {
    fn from_output(output: u8) -> Self {
        match output {
            b'#' => TileType::Scaffold,
            b'.' => TileType::Empty,
            b'^' | b'v' | b'<' | b'>' => TileType::Robot,
            _ => panic!("Unknown tile type: {}", output),
        }
    }
}

struct Computer {
    program: Vec<i64>,
    pc: usize,
    next_input: i64,
    last_output: i64,
    relative_offset: i64,
    halted: bool,
}

impl Computer {
    fn new(program: Vec<i64>) -> Self {
        Self {
            program,
            pc: 0,
            next_input: 0,
            last_output: 0,
            relative_offset: 0,
            halted: false,
        }
    }

    fn run_until_input(&mut self, input_val: i64) {
        self.next_input = input_val;
        self.run_until_op(3);
    }

    fn run_until_output(&mut self) -> i64 {
        self.run_until_op(4);
        self.last_output
    }

    fn run_until_halt(&mut self) {
        self.run_until_op(99);
    }

    fn run_until_op(&mut self, op_code: i64) {
        while !self.halted {
            let last_op = self.step();
            if last_op == op_code {
                break;
            }
        }
    }

    fn step(&mut self) -> i64 {
        let inst = self.program[self.pc];
        let op = inst % 100;
        let modes: [u8; 3] = [
            ((inst / 100) % 10) as u8,
            ((inst / 1000) % 10) as u8,
            ((inst / 10000) % 10) as u8,
        ];

        match op {
            1 | 2 | 7 | 8 => {
                let val_1 = self.get_val(&modes, 0);
                let val_2 = self.get_val(&modes, 1);

                let value = match op {
                    1 => val_1 + val_2,
                    2 => val_1 * val_2,
                    7 if val_1 < val_2 => 1,
                    7 => 0,
                    8 if val_1 == val_2 => 1,
                    8 => 0,
                    _ => unreachable!(),
                };

                let addr = self.get_addr(&modes, 2);
                self.program[addr] = value;
                self.pc += 4;
            }
            3 => {
                let addr = self.get_addr(&modes, 0);
                self.program[addr] = self.next_input;
                self.pc += 2;
            }
            4 => {
                let val = self.get_val(&modes, 0);
                self.last_output = val;
                self.pc += 2;
            }
            5 | 6 => {
                let val_1 = self.get_val(&modes, 0);
                let val_2 = self.get_val(&modes, 1);

                if op == 5 && val_1 != 0 || op == 6 && val_1 == 0 {
                    self.pc = val_2 as usize;
                } else {
                    self.pc += 3;
                }
            }
            9 => {
                let val = self.get_val(&modes, 0);
                self.relative_offset += val;
                self.pc += 2;
            }
            99 => self.halted = true,
            _ => panic!("Unknown op: {}", op),
        }

        op
    }

    fn get_val(&mut self, modes: &[u8; 3], instr_index: usize) -> i64 {
        let addr = self.get_addr(modes, instr_index);
        self.program[addr]
    }

    fn get_addr(&mut self, modes: &[u8; 3], instr_index: usize) -> usize {
        let pc_index = self.pc + 1 + instr_index;
        let addr = match modes[instr_index] {
            0 => self.program[pc_index] as usize,
            1 => pc_index,
            2 => (self.program[pc_index] + self.relative_offset) as usize,
            _ => panic!("Unknown address mode: {}", modes[instr_index]),
        };

        self.expand_program(addr);
        addr
    }

    fn expand_program(&mut self, addr: usize) {
        if addr >= self.program.len() {
            self.program.resize(addr + 1, 0);
        }
    }
}

fn main() {
    println!("{}", solve(true));
}

fn solve(is_v2: bool) -> i64 {
    let line = read_line().unwrap();
    let mut nums: Vec<i64> = line.split(",").map(|i| i.parse::<i64>().unwrap()).collect();

    if is_v2 {
        nums[0] = 2;
    }

    let mut c = Computer::new(nums);
    let mut tile_map: HashMap<Point, TileType> = HashMap::new();
    let mut curr_point = Point { x: 0, y: 0 };
    let mut robot_char = ' ';

    if is_v2 {
        // Manually built this by hand from P1 - don't give me that look
        // MAIN = A,B,A,C,C,A,B,C,B,B
        // A = L,8,R,10,L,8,R,8
        // B = L,12,R,8,R,8
        // C = L,8,R,6,R,6,R,10,L,8

        let main_fnc = "A,B,A,C,C,A,B,C,B,B\n";
        let a_fnc = "L,8,R,10,L,8,R,8\n";
        let b_fnc = "L,12,R,8,R,8\n";
        let c_fnc = "L,8,R,6,R,6,R,10,L,8\n";
        let input = format!("{}{}{}{}n\n", main_fnc, a_fnc, b_fnc, c_fnc);
        input
            .as_bytes()
            .iter()
            .for_each(|b| c.run_until_input(*b as i64));
        c.run_until_halt();
        c.last_output
    } else {
        loop {
            let output = c.run_until_output() as u8;
            if c.halted {
                break;
            }
            if output == b'\n' {
                curr_point.new_line();
                continue;
            }

            let tile = TileType::from_output(output);
            if tile == TileType::Robot {
                robot_char = output as char;
            }
            tile_map.insert(curr_point.clone(), tile);
            curr_point.next_char();
        }

        let points: Vec<&Point> = tile_map
            .iter()
            .filter(|(_, tile_type)| tile_type != &&TileType::Empty)
            .filter(|(point, _)| adj_scaffold_count(&tile_map, point) > 2)
            .map(|(point, _)| point)
            .collect();

        draw(&tile_map, &points, robot_char);
        points.iter().map(|p| i64::from(p.x * p.y)).sum()
    }
}

fn adj_scaffold_count(tile_map: &HashMap<Point, TileType>, point: &Point) -> usize {
    point
        .build_adj()
        .iter()
        .filter(|p| match tile_map.get(p) {
            Some(TileType::Scaffold) => true,
            _ => false,
        })
        .count()
}

fn draw(tile_map: &HashMap<Point, TileType>, intersections: &Vec<&Point>, robot_char: char) {
    let max_x = tile_map.keys().map(|p| p.x).max().unwrap();
    let max_y = tile_map.keys().map(|p| p.y).max().unwrap();

    let output = (0..=max_y)
        .map(|y| {
            (0..=max_x)
                .map(|x| {
                    let p = Point { x, y };
                    if intersections.contains(&&p) {
                        'O'
                    } else {
                        match tile_map.get(&&p) {
                            Some(TileType::Scaffold) => '#',
                            Some(TileType::Robot) => robot_char,
                            Some(TileType::Empty) => '.',
                            _ => ' ',
                        }
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", output);
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
