use std::collections::HashSet;
use std::io;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct RobotPos {
    pos: Point,
    facing: u8,
}

impl RobotPos {
    fn new() -> Self {
        Self {
            pos: Point { x: 0, y: 0 },
            facing: 0,
        }
    }

    fn turn_left(&mut self) {
        self.facing = (self.facing + 3) % 4
    }

    fn turn_right(&mut self) {
        self.facing = (self.facing + 1) % 4
    }

    fn move_forward(&mut self) {
        match self.facing {
            0 => self.pos.y -= 1,
            1 => self.pos.x += 1,
            2 => self.pos.y += 1,
            3 => self.pos.x -= 1,
            _ => unreachable!(),
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

    fn run_until_output(&mut self) {
        self.run_until_op(4);
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

fn solve(is_v2: bool) -> String {
    let line = read_line().unwrap();
    let nums: Vec<i64> = line.split(",").map(|i| i.parse::<i64>().unwrap()).collect();
    let mut white_cells: HashSet<Point> = HashSet::new();
    let mut painted_cells: HashSet<Point> = HashSet::new();
    let mut c = Computer::new(nums.clone());
    let mut robot_pos = RobotPos::new();

    if is_v2 {
        white_cells.insert(robot_pos.pos.clone());
    }

    while !c.halted {
        c.run_until_input(if white_cells.contains(&robot_pos.pos) {
            1
        } else {
            0
        });

        c.run_until_output();
        match c.last_output {
            0 => white_cells.remove(&robot_pos.pos),
            1 => white_cells.insert(robot_pos.pos.clone()),
            _ => panic!("Unknown colour!"),
        };

        painted_cells.insert(robot_pos.pos.clone());

        c.run_until_output();
        match c.last_output {
            0 => robot_pos.turn_left(),
            1 => robot_pos.turn_right(),
            _ => panic!("Unknown dir!"),
        };
        robot_pos.move_forward();
    }

    if is_v2 {
        let min_x = white_cells.iter().map(|p| p.x).min().unwrap();
        let min_y = white_cells.iter().map(|p| p.y).min().unwrap();
        let adjusted_cells = white_cells
            .iter()
            .map(|p| Point {
                x: p.x - min_x,
                y: p.y - min_y,
            })
            .collect::<HashSet<Point>>();
        let max_x = adjusted_cells.iter().map(|p| p.x).max().unwrap();
        let max_y = adjusted_cells.iter().map(|p| p.y).max().unwrap();

        (0..max_y + 1)
            .map(|y| {
                (0..max_x + 1)
                    .map(|x| {
                        if adjusted_cells.contains(&Point { x, y }) {
                            '0'
                        } else {
                            ' '
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    } else {
        painted_cells.len().to_string()
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
