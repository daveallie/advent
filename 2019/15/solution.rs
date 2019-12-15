use std::collections::HashMap;
use std::io;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn origin() -> Self {
        Point { x: 0, y: 0 }
    }

    fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    fn adjacent_points(&self) -> [(i64, Self); 4] {
        [
            (
                1,
                Self {
                    x: self.x,
                    y: self.y - 1,
                },
            ),
            (
                2,
                Self {
                    x: self.x,
                    y: self.y + 1,
                },
            ),
            (
                3,
                Self {
                    x: self.x - 1,
                    y: self.y,
                },
            ),
            (
                4,
                Self {
                    x: self.x + 1,
                    y: self.y,
                },
            ),
        ]
    }
}

#[derive(Clone, PartialEq, Debug)]
enum TileType {
    Empty,
    Wall,
    OxySys,
}

impl TileType {
    fn from_output(output: i64) -> Self {
        match output {
            0 => TileType::Wall,
            1 => TileType::Empty,
            2 => TileType::OxySys,
            _ => panic!("Unknown tile type"),
        }
    }
}

struct KnownTile {
    tile: TileType,
    dist: u32,
}

#[derive(Clone)]
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

#[derive(Clone)]
struct CompState {
    computer: Computer,
    curr_point: Point,
    curr_dist: u32,
}

impl CompState {
    fn initial_state(computer: Computer) -> Self {
        Self {
            computer,
            curr_dist: 0,
            curr_point: Point::origin(),
        }
    }
}

fn main() {
    println!("{}", solve(true));
}

fn solve(is_v2: bool) -> u32 {
    let line = read_line().unwrap();
    let nums: Vec<i64> = line.split(",").map(|i| i.parse::<i64>().unwrap()).collect();

    let start_comp_state = CompState::initial_state(Computer::new(nums));
    let (known_cells, mut final_comp_state) = bfs(start_comp_state, false);

    if is_v2 {
        final_comp_state.curr_point = Point::origin();
        final_comp_state.curr_dist = 0;
        let (known_cells, _) = bfs(final_comp_state, true);

        known_cells
            .values()
            .filter(|known_tile| known_tile.tile == TileType::Empty)
            .map(|known_tile| known_tile.dist)
            .max()
            .unwrap()
    } else {
        known_cells
            .values()
            .filter(|known_tile| known_tile.tile == TileType::OxySys)
            .map(|known_tile| known_tile.dist)
            .next()
            .unwrap()
    }
}

fn bfs(start_comp_state: CompState, is_v2: bool) -> (HashMap<Point, KnownTile>, CompState) {
    let mut final_comp_state = start_comp_state.clone();
    let mut bfs_states: Vec<CompState> = vec![start_comp_state];
    let mut known_cells: HashMap<Point, KnownTile> = HashMap::new();

    while bfs_states.len() > 0 {
        let mut comp_state = bfs_states.pop().unwrap();
        if known_cells.contains_key(&comp_state.curr_point) {
            continue;
        }

        let tile = if comp_state.curr_point.is_origin() {
            TileType::Empty
        } else {
            TileType::from_output(comp_state.computer.run_until_output())
        };
        known_cells.insert(
            comp_state.curr_point.clone(),
            KnownTile {
                tile: tile.clone(),
                dist: comp_state.curr_dist,
            },
        );

        match tile {
            TileType::OxySys if !is_v2 => {
                final_comp_state = comp_state;
                break;
            }
            TileType::Empty => {
                for (comp_input, new_point) in comp_state.curr_point.adjacent_points().into_iter() {
                    let mut new_comp = comp_state.computer.clone();
                    new_comp.run_until_input(*comp_input);
                    bfs_states.insert(
                        0,
                        CompState {
                            computer: new_comp,
                            curr_point: new_point.clone(),
                            curr_dist: comp_state.curr_dist + 1,
                        },
                    );
                }
            }
            _ => (),
        };
    }

    (known_cells, final_comp_state)
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
