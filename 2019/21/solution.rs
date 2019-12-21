use std::io;

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

    fn run_until_input_line(&mut self, input: &str) {
        input.bytes().for_each(|b| self.run_until_input(b as i64));
        self.run_until_input(b'\n' as i64);
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

fn main() {
    solve(true);
}

fn solve(is_v2: bool) {
    let line = read_line().unwrap();
    let nums: Vec<i64> = line.split(",").map(|i| i.parse::<i64>().unwrap()).collect();
    let mut c = Computer::new(nums);

    // !A || ((!B || !C) && D[ && (E || H)])  [v2 only]
    // v1 = next space hold or jumpable hole with safe spot
    // v2 = next space hole or jumpable hole with safe spot && step or double jump
    c.run_until_input_line("NOT B T");
    c.run_until_input_line("NOT C J");
    c.run_until_input_line("OR T J");
    c.run_until_input_line("AND D J");

    if is_v2 {
        c.run_until_input_line("NOT E T");
        c.run_until_input_line("NOT T T");
        c.run_until_input_line("OR H T");
        c.run_until_input_line("AND T J");
    }

    c.run_until_input_line("NOT A T");
    c.run_until_input_line("OR T J");

    if is_v2 {
        c.run_until_input_line("RUN");
    } else {
        c.run_until_input_line("WALK");
    }

    loop {
        let output = c.run_until_output();
        if c.halted {
            break;
        }

        if output == b'\n' as i64 {
            println!();
            continue;
        }

        if output < std::u8::MAX as i64 {
            print!("{}", (output as u8) as char);
        } else {
            println!("{}", output);
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
