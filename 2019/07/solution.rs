use std::io;

struct PhasePermIterator {
    elems: Vec<i32>,
}

impl PhasePermIterator {
    fn new(is_v2: bool) -> Self {
        let elems = if is_v2 {
            vec![5, 6, 7, 8, 9]
        } else {
            vec![0, 1, 2, 3, 4]
        };
        Self { elems }
    }
}

impl Iterator for PhasePermIterator {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Vec<i32>> {
        let mut i = self.elems.len() - 1;
        while i > 0 && self.elems[i - 1] >= self.elems[i] {
            i -= 1;
        }

        if i == 0 {
            return None;
        }

        let mut j = self.elems.len() - 1;
        while j >= i && self.elems[j] <= self.elems[i - 1] {
            j -= 1;
        }

        self.elems.swap(j, i - 1);
        self.elems[i..].reverse();

        Some(self.elems.clone())
    }
}

struct Amplifier {
    program: Vec<i32>,
    pc: usize,
    next_input: i32,
    last_output: i32,
    halted: bool,
}

impl Amplifier {
    fn new(program: Vec<i32>) -> Self {
        Self {
            program,
            pc: 0,
            next_input: 0,
            last_output: 0,
            halted: false,
        }
    }

    fn create_and_prime(program: Vec<i32>, phase: i32) -> Self {
        let mut a = Self::new(program);
        a.run_until_input(phase);
        a
    }

    fn run_until_input(&mut self, input_val: i32) {
        self.next_input = input_val;
        self.run_until_op(3);
    }

    fn run_until_output(&mut self) {
        self.run_until_op(4);
    }

    fn run_until_op(&mut self, op_code: i32) {
        while !self.halted {
            let last_op = self.step();
            if last_op == op_code {
                break;
            }
        }
    }

    fn step(&mut self) -> i32 {
        let inst = self.program[self.pc];
        let op = inst % 100;
        let modes: [bool; 2] = [(inst / 100) % 2 == 1, (inst / 1000) % 2 == 1];

        match op {
            1 | 2 | 7 | 8 => {
                let val_1 = self.get_val(modes[0], self.program[self.pc + 1]);
                let val_2 = self.get_val(modes[1], self.program[self.pc + 2]);

                let value = match op {
                    1 => val_1 + val_2,
                    2 => val_1 * val_2,
                    7 if val_1 < val_2 => 1,
                    7 => 0,
                    8 if val_1 == val_2 => 1,
                    8 => 0,
                    _ => unreachable!(),
                };

                let addr = self.program[self.pc + 3] as usize;
                self.program[addr] = value;
                self.pc += 4;
            }
            3 => {
                let addr = self.program[self.pc + 1] as usize;
                self.program[addr] = self.next_input;
                self.pc += 2;
            }
            4 => {
                let val_1 = self.get_val(modes[0], self.program[self.pc + 1]);
                self.last_output = val_1;
                self.pc += 2
            }
            5 | 6 => {
                let val_1 = self.get_val(modes[0], self.program[self.pc + 1]);
                let val_2 = self.get_val(modes[1], self.program[self.pc + 2]);

                if op == 5 && val_1 != 0 || op == 6 && val_1 == 0 {
                    self.pc = val_2 as usize;
                } else {
                    self.pc += 3;
                }
            }
            99 => self.halted = true,
            _ => panic!("Unknown op: {}", op),
        }

        op
    }

    fn get_val(&self, immediate_mode: bool, val: i32) -> i32 {
        if immediate_mode {
            val
        } else {
            self.program[val as usize]
        }
    }
}

fn main() {
    println!("{}", solve(true));
}

fn solve(is_v2: bool) -> i32 {
    let line = read_line().unwrap();
    let nums: Vec<i32> = line.split(",").map(|i| i.parse::<i32>().unwrap()).collect();

    PhasePermIterator::new(is_v2)
        .map(|phases| {
            let mut amplifiers: Vec<Amplifier> = phases
                .iter()
                .map(|phase| Amplifier::create_and_prime(nums.clone(), *phase))
                .collect();
            let mut last_engine_output = 0;

            loop {
                last_engine_output =
                    amplifiers
                        .iter_mut()
                        .fold(last_engine_output, |power, amp| {
                            amp.run_until_input(power);
                            amp.run_until_output();
                            amp.last_output
                        });

                // Only one iteration for v1
                if !is_v2 || amplifiers.iter().any(|a| a.halted) {
                    break;
                }
            }

            last_engine_output
        })
        .max()
        .unwrap()
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
