use std::collections::HashMap;
use std::{io, thread, time};

#[derive(Hash, Eq, PartialEq, Clone)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(PartialEq)]
enum TileType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TileType {
    fn from_id(tile_id: i64) -> Self {
        match tile_id {
            0 => TileType::Empty,
            1 => TileType::Wall,
            2 => TileType::Block,
            3 => TileType::Paddle,
            4 => TileType::Ball,
            _ => panic!("Unknown tile type: {}", tile_id),
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

fn solve(is_v2: bool) -> i64 {
    let line = read_line().unwrap();
    let mut nums: Vec<i64> = line.split(",").map(|i| i.parse::<i64>().unwrap()).collect();
    if is_v2 {
        nums[0] = 2;
    }

    let render_sleep = time::Duration::from_millis(20);
    let rendering_enabled = false;
    let mut c = Computer::new(nums);
    let mut tiles: HashMap<Point, TileType> = HashMap::new();
    let mut paddle_pos: Option<Point> = None;
    let mut score = 0;

    loop {
        c.run_until_output();
        let x = c.last_output;
        c.run_until_output();
        let y = c.last_output;
        c.run_until_output();
        let tile_id = c.last_output;

        if c.halted {
            break;
        }

        if x == -1 && y == 0 {
            score = tile_id;
            continue;
        }

        let point = Point { x, y };
        let tile = TileType::from_id(tile_id);

        if tile == TileType::Empty {
            tiles.remove(&point);
            continue;
        }

        if block_count(&tiles) > 0 {
            match tile {
                TileType::Ball => match paddle_pos.clone() {
                    Some(p_pos) => {
                        c.next_input = if point.x < p_pos.x {
                            -1
                        } else if p_pos.x < point.x {
                            1
                        } else {
                            0
                        };
                    }
                    _ => (),
                },
                TileType::Paddle => paddle_pos = Some(point.clone()),
                _ => (),
            }
        }

        tiles.insert(point, tile);

        if rendering_enabled && paddle_pos.is_some() {
            render(&tiles, score);
            thread::sleep(render_sleep);
        }
    }

    if is_v2 {
        score
    } else {
        block_count(&tiles) as i64
    }
}

fn block_count(tiles: &HashMap<Point, TileType>) -> usize {
    tiles.values().filter(|t| t == &&TileType::Block).count()
}

fn render(tiles: &HashMap<Point, TileType>, score: i64) {
    println!("\nScore: {}", score);
    let max_x = tiles.keys().map(|p| p.x).max().unwrap();
    let max_y = tiles.keys().map(|p| p.y).max().unwrap();

    let output = (0..max_y + 1)
        .map(|y| {
            (0..max_x + 1)
                .map(|x| match tiles.get(&Point { x, y }) {
                    Some(TileType::Wall) => '#',
                    Some(TileType::Block) => 'x',
                    Some(TileType::Paddle) => '_',
                    Some(TileType::Ball) => '+',
                    _ => ' ',
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
