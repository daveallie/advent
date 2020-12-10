use std::collections::HashSet;

struct Console {
    pc: usize,
    seen_pcs: HashSet<usize>,
    acc: i32,
    rom: Vec<Instruction>,
}

impl Console {
    fn new(rom: Vec<Instruction>) -> Self {
        Self {
            pc: 0,
            seen_pcs: HashSet::new(),
            acc: 0,
            rom,
        }
    }

    fn is_terminated(&self) -> bool {
        self.pc >= self.rom.len()
    }

    fn is_looping(&self) -> bool {
        self.seen_pcs.contains(&self.pc)
    }

    fn step(&mut self) {
        self.seen_pcs.insert(self.pc);
        let inst = self.rom.get(self.pc).unwrap();
        self.pc += 1;

        match inst.op {
            Operation::NOP => {},
            Operation::ACC => self.acc += inst.val,
            Operation::JMP => self.pc = (self.pc as i32 + inst.val - 1) as usize,
        }
    }
}

#[derive(Clone, PartialEq)]
enum Operation {
    NOP,
    ACC,
    JMP,
}

#[derive(Clone)]
struct Instruction {
    op: Operation,
    val: i32,
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines()
        .map(|line| {
            let mut line_parts = line.split_whitespace();
            let op = match line_parts.next().unwrap() {
                "nop" => Operation::NOP,
                "acc" => Operation::ACC,
                "jmp" => Operation::JMP,
                _ => panic!("Unknown operation")
            };

            let val = line_parts.next().unwrap().parse().unwrap();

            Instruction {
                op,
                val,
            }
        }).collect()
}

#[aoc(day8, part1)]
fn solve_part1(rom: &[Instruction]) -> i32 {
    let rom = rom.to_vec();
    let mut console = Console::new(rom);

    while !console.is_looping() {
        console.step();
    }

    console.acc
}

#[aoc(day8, part2)]
fn solve_part2(rom: &[Instruction]) -> i32 {
    let original_rom = rom.to_vec();
    let mut start_index = 0;

    loop {
        let mut rom = original_rom.clone();
        start_index = mutate_rom(&mut rom, start_index);
        let mut console = Console::new(rom);

        while !console.is_terminated() && !console.is_looping() {
            console.step();
        }

        if console.is_terminated() {
            return console.acc
        }
    }
}

fn mutate_rom(rom: &mut Vec<Instruction>, start_index: usize) -> usize {
    for index in start_index..rom.len() {
        let inst = rom.get_mut(index).unwrap();

        if inst.op == Operation::JMP {
            inst.op = Operation::NOP;
            return index + 1;
        } else if inst.op == Operation::NOP {
            inst.op = Operation::JMP;
            return index + 1;
        }
    }

    0
}
